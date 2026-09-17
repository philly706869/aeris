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
                mapped_order.push(value);
                let offset = Index::from(offset);
                mapped_groups[group].push(quote!(#values.#offset));
                order.push((group, index));
            }
            let mapped_order = sequence_tasks(mapped_order, &mapping_names);
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
                quote!(::xst::internal::MappingTask::defer(move || {
                    #node.sequence(#count)?;
                    ::xst::internal::Result::Ok(#mapped_order.map(move |#values| {
                        ::xst::internal::Result::Ok(#ident {
                            #marker_ident: ::xst::internal::PhantomData,
                            #(#mapped_fields,)*
                        })
                    }))
                })),
            )
        }
        ir::Kind::Enum(variants) => {
            let variant_index = &mapping_names[2];
            let mapped_arms = variants
                .iter()
                .enumerate()
                .map(|(index, (variant, _, mapping))| {
                    let value = emit_mapping(mapping, quote!(#node), &mapping_names);
                    quote!(#index => #value.map(move |#node| ::xst::internal::Result::Ok(#ident::#variant(#node))))
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
                quote!(::xst::internal::MappingTask::defer(move || {
                    let (#variant_index, #node) = #node.alternative()?;
                    ::xst::internal::Result::Ok(match #variant_index {
                        #(#mapped_arms,)*
                        _ => ::xst::internal::MappingTask::ready(::xst::internal::Result::Err(::xst::internal::ExtractError::InvalidMapping)),
                    })
                })),
            )
        }
        ir::Kind::Type => (
            quote!(#vis struct #ident<'i, #bounds>(::xst::internal::PhantomData<&'i ()>);),
            quote!(),
            quote!(&'i ::xst::internal::str),
            quote!(::xst::internal::MappingTask::ready(#node.slice())),
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
                fn map<'a, 'i: 'a>(#node: ::xst::internal::MappingNode<'a, 'i>) -> ::xst::internal::MappingTask<'a, Self::Output<'i>> {
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
                fn map<'a, 'i: 'a>(#node: ::xst::internal::MappingNode<'a, 'i>) -> ::xst::internal::MappingTask<'a, Self::Output<'i>> {
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
        ir::Mapping::Slice => quote!(::xst::internal::MappingTask::ready(#node.slice())),
        ir::Mapping::Reference(ty) => quote!(#node.reference_task::<#ty>()),
        ir::Mapping::Box(inner) => {
            let value = emit_mapping(inner, quote!(#node), names);
            quote!(#value.map(move |#node| ::xst::internal::Result::Ok(::xst::internal::Box::new(#node))))
        }
        ir::Mapping::Option(inner) => {
            let value = emit_mapping(inner, quote!(#node), names);
            quote!(match #node.optional()? {
                ::xst::internal::Option::Some(#node) => #value.map(move |#node| ::xst::internal::Result::Ok(::xst::internal::Option::Some(#node))),
                ::xst::internal::Option::None => ::xst::internal::MappingTask::ready(::xst::internal::Result::Ok(::xst::internal::Option::None)),
            })
        }
        ir::Mapping::Vec(inner) => {
            let values = &names[3];
            let value = emit_mapping(inner, quote!(#node), names);
            quote!({
                let mut #values = ::xst::internal::Vec::new();
                for #node in #node.repeated()? { #values.push(#value); }
                ::xst::internal::MappingTask::collect(#values)
            })
        }
        ir::Mapping::Tuple(items) => {
            let count = items.len();
            let values = items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    emit_mapping(item, quote!(#node.field(#index, #count)?), names)
                })
                .collect();
            let sequence = sequence_tasks(values, names);
            quote!({ #node.sequence(#count)?; #sequence })
        }
    };
    quote!(::xst::internal::MappingTask::defer(move || {
        let #node = #input;
        ::xst::internal::Result::Ok(#body)
    }))
}

// Zip statically typed heterogeneous fields, then flatten their nested tuple.
// Each combinator schedules its child; it never recursively runs it.
fn sequence_tasks(tasks: Vec<TokenStream>, names: &[Ident; 4]) -> TokenStream {
    let count = tasks.len();
    let mut sequence = quote!(::xst::internal::MappingTask::ready(
        ::xst::internal::Result::Ok(())
    ));
    for task in tasks {
        sequence = quote!(#sequence.zip(#task));
    }
    let value = &names[1];
    let fields = (0..count).map(|index| {
        let mut field = quote!(#value);
        for _ in index + 1..count {
            field = quote!(#field.0);
        }
        quote!(#field.1)
    });
    quote!(#sequence.map(move |#value| ::xst::internal::Result::Ok((#(#fields,)*))))
}
