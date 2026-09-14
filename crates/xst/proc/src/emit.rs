use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Index};

use crate::ir::{Kind, Shard};

pub fn emit(shard: Shard) -> TokenStream {
    let Shard {
        vis,
        ident,
        params,
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
    let (main, debug, output) = match kind {
        Kind::Struct(fields) => {
            // Group storage by name, but retain declaration order for Debug and DATA.
            let mut groups: Vec<(Ident, Vec<TokenStream>)> = Vec::new();
            let mut order = Vec::new();
            for (ident, ty) in fields {
                let group = groups
                    .iter()
                    .position(|(name, _)| name == &ident)
                    .unwrap_or_else(|| {
                        groups.push((ident.clone(), Vec::new()));
                        groups.len() - 1
                    });
                let index = groups[group].1.len();
                groups[group].1.push(ty);
                order.push((group, index));
            }
            let mut marker_index = 0;
            let marker_ident = loop {
                let candidate = format_ident!("__xst_marker_{marker_index}");
                if groups.iter().all(|(ident, _)| ident != &candidate) {
                    break candidate;
                }
                marker_index += 1;
            };
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
            )
        }
        Kind::Enum(variants) => {
            let declarations = variants.iter().map(|(ident, ty)| quote!(#ident(#ty)));
            let arms = variants.iter().map(|(ident, _)| {
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
            )
        }
        Kind::Type => (
            quote!(#vis struct #ident<'i, #bounds>(::xst::internal::PhantomData<&'i ()>);),
            quote!(),
            quote!(&'i ::xst::internal::str),
        ),
    };
    let static_shard = params.is_empty().then(|| {
        quote! {
            impl ::xst::internal::StaticShard for #ident<'static> {}
        }
    });
    let closures = closures.into_iter().map(|closure| {
        let name = format_ident!("__xst_shard_closure_{}", closure.index);
        let index = Index::from(closure.index);
        let output = closure.output;
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
            }
            impl #generics ::xst::internal::ShardClosureForward<#index> for __xst_shard_core_0 #args {
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
                type Core = __xst_shard_core_0 #args;
            }
            #[allow(non_camel_case_types)]
            #vis struct __xst_shard_core_0 #generics (#marker);
            impl #generics ::xst::internal::ShardCore for __xst_shard_core_0 #args {
                type Output<'i> = #output;
                const DATA: &'static ::xst::internal::ShardData = &#data;
            }
            #(#closures)*
        };
    }
}
