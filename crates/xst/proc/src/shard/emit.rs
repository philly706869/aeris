use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Index};

use crate::shard::ir;

pub fn emit(shard: ir::Shard) -> TokenStream {
    let ir::Shard {
        vis,
        ident,
        params,
        core_ident,
        marker_ident,
        mapping_names,
        kind,
        data,
        closures,
    } = shard;
    let bounds = quote!(#(#params: ::xst::internal::Shard),*);
    let generics = if params.is_empty() {
        quote!()
    } else {
        quote!(<#bounds>)
    };
    let args = if params.is_empty() {
        quote!()
    } else {
        quote!(<#(#params),*>)
    };
    let marker = quote!(::xst::internal::PhantomData<fn() -> (#(#params,)*)>);
    let name = ident.to_string();
    let node = &mapping_names[0];
    let (main, debug, output, mapping) = match kind {
        ir::Kind::Struct(fields) => {
            let mut groups: Vec<(Ident, Vec<TokenStream>)> = Vec::new();
            let mut order = Vec::new();
            let mut mapped_groups: Vec<Vec<TokenStream>> = Vec::new();
            let mut mapped_order = Vec::new();
            let values = &mapping_names[3];
            let count = fields.len();
            for (offset, (ident, ty, mapping)) in fields.into_iter().enumerate() {
                let group = groups
                    .iter()
                    .position(|(name, _)| name == &ident)
                    .unwrap_or_else(|| {
                        groups.push((ident.clone(), Vec::new()));
                        mapped_groups.push(Vec::new());
                        groups.len() - 1
                    });
                let index = groups[group].1.len();
                groups[group].1.push(ty);
                let value = emit_mapping(
                    &mapping,
                    quote!(#node.field(#offset, #count)?),
                    &mapping_names,
                );
                mapped_order.push(quote!(#value?));
                let offset = Index::from(offset);
                mapped_groups[group].push(quote!(#values.#offset));
                order.push((group, index));
            }
            let mapped_fields = groups
                .iter()
                .zip(&mapped_groups)
                .map(|((ident, _), values)| {
                    if values.len() == 1 {
                        let value = &values[0];
                        quote!(#ident: #value)
                    } else {
                        quote!(#ident: (#(#values,)*))
                    }
                });
            let fields = groups.iter().map(|(ident, types)| {
                if types.len() == 1 {
                    let ty = &types[0];
                    quote!(#ident: #ty)
                } else {
                    quote!(#ident: (#(#types,)*))
                }
            });
            let debug_fields = order.iter().map(|&(group, index)| {
                let (ident, types) = &groups[group];
                let name = ident.to_string();
                if types.len() == 1 {
                    quote!(.field(#name, &self.#ident))
                } else {
                    let index = Index::from(index);
                    quote!(.field(#name, &self.#ident.#index))
                }
            });
            (
                quote! {
                    #vis struct #ident<'i, #bounds> {
                        #marker_ident: ::xst::internal::PhantomData<&'i ()>,
                        #(#fields,)*
                    }
                },
                quote! {
                    impl<'i, #bounds> ::xst::internal::fmt::Debug for #ident<'i, #(#params),*> {
                        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
                            f.debug_struct(#name) #(#debug_fields)* .finish()
                        }
                    }
                },
                quote!(#ident<'i, #(#params),*>),
                quote!({
                    #node.sequence(#count)?;
                    let #values = (#(#mapped_order,)*);
                    ::xst::internal::Result::Ok(#ident {
                        #marker_ident: ::xst::internal::PhantomData,
                        #(#mapped_fields,)*
                    })
                }),
            )
        }
        ir::Kind::Enum(variants) => {
            let variant_index = &mapping_names[2];
            let mapped_arms = variants
                .iter()
                .enumerate()
                .map(|(index, (variant, _, mapping))| {
                    let value = emit_mapping(mapping, quote!(#node), &mapping_names);
                    quote!(#index => ::xst::internal::Result::Ok(#ident::#variant(#value?)))
                });
            let declarations = variants.iter().map(|(ident, ty, _)| quote!(#ident(#ty)));
            let arms = variants.iter().map(|(ident, _, _)| {
                let name = ident.to_string();
                quote!(Self::#ident(value) => f.debug_tuple(#name).field(value).finish())
            });
            (
                quote!(#vis enum #ident<'i, #bounds> { #(#declarations,)* }),
                quote! {
                    impl<'i, #bounds> ::xst::internal::fmt::Debug for #ident<'i, #(#params),*> {
                        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
                            match self { #(#arms,)* }
                        }
                    }
                },
                quote!(#ident<'i, #(#params),*>),
                quote!({
                    let (#variant_index, #node) = #node.alternative()?;
                    match #variant_index {
                        #(#mapped_arms,)*
                        _ => ::xst::internal::Result::Err(::xst::internal::ExtractError::InvalidMapping),
                    }
                }),
            )
        }
        ir::Kind::Type => (
            quote!(#vis struct #ident<'i, #bounds>(::xst::internal::PhantomData<&'i ()>);),
            quote!(),
            quote!(&'i ::xst::internal::str),
            quote!(#node.slice()),
        ),
    };
    let static_shard = params.is_empty().then(|| {
        quote! {
            impl ::xst::internal::StaticShard for #ident<'static> {}
        }
    });
    let closures = closures.into_iter().map(|closure| {
        let name = closure.ident;
        let index = Index::from(closure.index);
        let output = closure.output;
        let mapping = emit_mapping(&closure.mapping, quote!(#node), &mapping_names);
        let data = closure.data;
        quote! {
            #[allow(non_camel_case_types)]
            #vis struct #name #generics (#marker);
            impl #generics ::xst::internal::Shard for #name #args {
                type Core = Self;
            }
            impl #generics ::xst::internal::ShardCore for #name #args {
                type Output<'i> = #output;
                const DATA: &'static ::xst::internal::ShardData = &#data;
                fn map<'i>(#node: ::xst::internal::MappingNode<'_, 'i>) -> ::xst::internal::Result<Self::Output<'i>, ::xst::internal::ExtractError> {
                    #mapping
                }
            }
            impl #generics ::xst::internal::ShardClosureForward<#index> for #core_ident #args {
                type Closure = #name #args;
            }
        }
    });
    quote! {
        #main
        const _: () = {
            #debug
            #static_shard
            impl #generics ::xst::internal::Shard for #ident<'static, #(#params),*> {
                type Core = #core_ident #args;
            }
            #[allow(non_camel_case_types)]
            #vis struct #core_ident #generics (#marker);
            impl #generics ::xst::internal::ShardCore for #core_ident #args {
                type Output<'i> = #output;
                const DATA: &'static ::xst::internal::ShardData = &#data;
                fn map<'i>(#node: ::xst::internal::MappingNode<'_, 'i>) -> ::xst::internal::Result<Self::Output<'i>, ::xst::internal::ExtractError> {
                    #mapping
                }
            }
            #(#closures)*
        };
    }
}

fn emit_mapping(mapping: &ir::Mapping, input: TokenStream, names: &[Ident; 4]) -> TokenStream {
    let node = &names[1];
    let body = match mapping {
        ir::Mapping::Slice => quote!(#node.slice()),
        ir::Mapping::Reference(ty) => quote!(#node.reference::<#ty>()),
        ir::Mapping::Box(inner) => {
            let value = emit_mapping(inner, quote!(#node), names);
            quote!(::xst::internal::Result::Ok(::xst::internal::Box::new(#value?)))
        }
        ir::Mapping::Option(inner) => {
            let value = emit_mapping(inner, quote!(#node), names);
            quote!(match #node.optional()? {
                ::xst::internal::Option::Some(#node) => ::xst::internal::Result::Ok(::xst::internal::Option::Some(#value?)),
                ::xst::internal::Option::None => ::xst::internal::Result::Ok(::xst::internal::Option::None),
            })
        }
        ir::Mapping::Vec(inner) => {
            let values = &names[3];
            let value = emit_mapping(inner, quote!(#node), names);
            quote!({
                let mut #values = ::xst::internal::Vec::new();
                for #node in #node.repeated()? { #values.push(#value?); }
                ::xst::internal::Result::Ok(#values)
            })
        }
        ir::Mapping::Tuple(items) => {
            let count = items.len();
            let values = items.iter().enumerate().map(|(index, item)| {
                let value = emit_mapping(item, quote!(#node.field(#index, #count)?), names);
                quote!(#value?)
            });
            quote!({ #node.sequence(#count)?; ::xst::internal::Result::Ok((#(#values,)*)) })
        }
    };
    quote!({ let #node = #input; #body })
}
