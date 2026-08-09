mod ast;
mod ir;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemMod, MacroDelimiter, Type, Visibility, parse, spanned::Spanned};

#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut errs = Vec::new();

    if !attr.is_empty() {
        let attr: proc_macro2::TokenStream = attr.into();
        errs.push(syn::Error::new_spanned(
            attr,
            "unexpected attribute argument",
        ));
    }

    match parse::<ItemMod>(item) {
        Ok(module) => {
            if let Some(unsafety) = module.unsafety {
                errs.push(syn::Error::new_spanned(
                    unsafety,
                    "unsafe shard is not supported",
                ))
            }

            errs.extend(
                module
                    .attrs
                    .into_iter()
                    .map(|attr| syn::Error::new_spanned(attr, "attribute is not supported")),
            );

            match (module.content, module.semi) {
                (Some((_, items)), None) => {
                    for item in items {
                        match item {
                            Item::Struct(item) => {
                                if item.ident != "Shard" {
                                    errs.push(syn::Error::new_spanned(item, "struct declaration except with name Shard is not supported in shard"));
                                } else {
                                    if item.vis != Visibility::Inherited {
                                        errs.push(syn::Error::new_spanned(
                                            &item.vis,
                                            "visibility is not supported in shard",
                                        ));
                                    }
                                    if !item.generics.params.is_empty() {
                                        errs.push(syn::Error::new_spanned(
                                            &item.generics,
                                            "generic is not supported in shard",
                                        ));
                                    }
                                    if item.generics.where_clause.is_some() {
                                        errs.push(syn::Error::new_spanned(
                                            &item.generics.where_clause,
                                            "where clause is not supported in shard",
                                        ));
                                    }
                                }
                            }
                            Item::Enum(item) => {
                                if item.ident != "Shard" {
                                    errs.push(syn::Error::new_spanned(item, "enum declaration except with name Shard is not supported in shard"));
                                } else {
                                    if item.vis != Visibility::Inherited {
                                        errs.push(syn::Error::new_spanned(
                                            &item.vis,
                                            "visibility is not supported in shard",
                                        ));
                                    }
                                    if !item.generics.params.is_empty() {
                                        errs.push(syn::Error::new_spanned(
                                            &item.generics,
                                            "generic is not supported in shard",
                                        ));
                                    }
                                    if item.generics.where_clause.is_some() {
                                        errs.push(syn::Error::new_spanned(
                                            &item.generics.where_clause,
                                            "where clause is not supported in shard",
                                        ));
                                    }
                                }
                            }
                            Item::Impl(item) => match item.self_ty.as_ref() {
                                Type::Path(path)
                                    if {
                                        let segments = &path.path.segments;
                                        segments.len() == 1 && segments[0].ident == "Shard"
                                    } =>
                                {
                                    if let Some(unsafety) = item.unsafety {
                                        errs.push(syn::Error::new_spanned(
                                            unsafety,
                                            "unsafe is not supported in shard",
                                        ));
                                    }
                                    if !item.generics.params.is_empty() {
                                        errs.push(syn::Error::new_spanned(
                                            &item.generics,
                                            "generic is not supported in shard",
                                        ));
                                    }
                                    if let Some(where_clause) = item.generics.where_clause {
                                        errs.push(syn::Error::new_spanned(
                                            where_clause,
                                            "where clause is not supported in shard",
                                        ));
                                    }
                                }
                                _ => {
                                    errs.push(syn::Error::new_spanned(item, "impl declaration except with name Shard is not supported in shard"));
                                }
                            },
                            Item::Macro(item) => {
                                if let Some(ident) = item.ident {
                                    errs.push(syn::Error::new_spanned(
                                        ident,
                                        "ident is not supported in shard",
                                    ));
                                }
                                match item.mac.delimiter {
                                    MacroDelimiter::Brace(_) => {}
                                    MacroDelimiter::Bracket(bracket) => {
                                        errs.push(syn::Error::new(
                                            bracket.span.span(),
                                            "", // TODO
                                        ));
                                    }
                                    MacroDelimiter::Paren(paren) => {
                                        errs.push(syn::Error::new(
                                            paren.span.span(),
                                            "", // TODO
                                        ));
                                    }
                                }
                            }
                            Item::Const(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "const declaration is not supported in shard",
                            )),
                            Item::ExternCrate(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "extern crate declaration is not supported in shard",
                            )),
                            Item::Fn(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "function declaration is not supported in shard",
                            )),
                            Item::ForeignMod(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "extern module declaration is not supported in shard",
                            )),
                            Item::Mod(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "module declaration is not supported in shard",
                            )),
                            Item::Static(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "static declaration is not supported in shard",
                            )),
                            Item::Trait(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "trait declaration is not supported in shard",
                            )),
                            Item::TraitAlias(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "trait alias declaration is not supported in shard",
                            )),
                            Item::Type(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "type alias declaration is not supported in shard",
                            )),
                            Item::Union(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "union declaration is not supported in shard",
                            )),
                            Item::Use(item) => errs.push(syn::Error::new_spanned(
                                item,
                                "use declaration is not supported in shard",
                            )),
                            _ => errs.push(syn::Error::new_spanned(
                                item,
                                "item is not supported in shard",
                            )),
                        }
                    }
                }
                (None, Some(semi)) => {
                    errs.push(syn::Error::new_spanned(semi, "unexpected token"));
                }
                _ => unreachable!(),
            }
        }
        Err(err) => errs.push(err),
    }

    match errs.into_iter().reduce(|mut acc, curr| {
        acc.combine(curr);
        acc
    }) {
        Some(err) => err.into_compile_error(),
        None => quote! {},
    }
    .into()
}
