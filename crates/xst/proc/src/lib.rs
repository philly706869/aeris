use proc_macro::TokenStream;

mod ast;
mod emit;
mod ir;
mod lower;

/// Defines a shard's output type and its compile-time grammar.
///
/// Structs map fields in declaration order, enums map alternatives, and type
/// declarations map matched text to a borrowed string. Generic parameters must
/// implement `Shard`; only declarations without parameters implement `StaticShard`.
#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "shard does not accept arguments",
        )
        .into_compile_error()
        .into();
    }
    let shard = syn::parse_macro_input!(item as ast::Shard);
    match lower::lower(shard) {
        Ok(shard) => emit::emit(shard).into(),
        Err(error) => error.into_compile_error().into(),
    }
}

/// # XST x! rune
#[proc_macro]
pub fn x(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xbox! rune
#[proc_macro]
pub fn xbox(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xopt! rune
#[proc_macro]
pub fn xopt(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xvec! rune
#[proc_macro]
pub fn xvec(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xoptz! rune
#[proc_macro]
pub fn xoptz(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xvecz! rune
#[proc_macro]
pub fn xvecz(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn expand(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
        lower::lower(syn::parse2(input)?).map(emit::emit)
    }

    #[test]
    fn rejects_malformed_nested_input() {
        for input in [
            quote!(
                struct A {
                    field: xbox![x! { "a" } garbage],
                }
            ),
            quote!(
                struct A {
                    field: x! { "a" @ },
                }
            ),
            quote!(
                type A = x! { {'a'..} };
            ),
            quote!(
                struct A {
                    field: xvec![x! { "a" }, 1..3 junk],
                }
            ),
            quote!(enum A { V(B extra) }),
        ] {
            assert!(expand(input.clone()).is_err(), "accepted {input}");
        }
    }

    #[test]
    fn rejects_invalid_ranges_and_parameters() {
        for input in [
            quote!(
                type A = x! { {'z'..'a'} };
            ),
            quote!(
                type A = x! { "a"^[4..2] };
            ),
            quote!(
                struct A {
                    field: xvec![B, 4..2],
                }
            ),
            quote!(
                struct A<T, T> {
                    field: T,
                }
            ),
            quote!(
                struct A<T> {
                    field: T<B>,
                }
            ),
            quote!(
                enum A {
                    V(B),
                    V(C),
                }
            ),
        ] {
            assert!(expand(input.clone()).is_err(), "accepted {input}");
        }
    }

    #[test]
    fn only_nongeneric_shards_are_static() {
        for input in [
            quote!(
                struct A<T> {
                    field: T,
                }
            ),
            quote!(
                enum A<T> {
                    V(T),
                }
            ),
            quote!(
                type A<T> = x! { T };
            ),
        ] {
            assert!(!expand(input).unwrap().to_string().contains("StaticShard"));
        }
        for input in [
            quote!(
                struct A {
                    field: x! { "a" },
                }
            ),
            quote!(
                enum A {
                    V(B),
                }
            ),
            quote!(
                type A = x! { "a" };
            ),
        ] {
            let tokens = expand(input).unwrap();
            assert_eq!(tokens.to_string().matches("StaticShard").count(), 1);
            // All auxiliary declarations and implementations live in the final const block.
            let trees: Vec<_> = tokens.into_iter().collect();
            let const_index = trees
                .iter()
                .position(|tree| tree.to_string() == "const")
                .unwrap();
            assert!(
                !trees[..const_index]
                    .iter()
                    .any(|tree| tree.to_string() == "impl")
            );
            assert!(
                matches!(&trees[trees.len() - 2], proc_macro2::TokenTree::Group(group) if group.delimiter() == proc_macro2::Delimiter::Brace)
            );
        }
    }

    #[test]
    fn unused_enum_parameters_are_left_to_rust() {
        assert!(
            expand(quote!(
                enum Empty {}
            ))
            .is_ok()
        );
        assert!(
            expand(quote!(
                enum Unused<T> {
                    Value(B),
                }
            ))
            .is_ok()
        );
    }

    #[test]
    fn z_and_question_modifiers_preserve_data() {
        fn data(input: proc_macro2::TokenStream) -> String {
            lower::lower(syn::parse2(input).unwrap())
                .unwrap()
                .data
                .to_string()
        }
        assert_eq!(
            data(quote!(
                struct A {
                    a: xopt![B],
                    b: xvec![B, 1..3],
                }
            )),
            data(quote!(
                struct A {
                    a: xoptz![B],
                    b: xvecz![B, 1..3],
                }
            )),
        );
        assert_eq!(
            data(quote!(
                type A = x! { "a"* "b"+ "c"^[1..3] };
            )),
            data(quote!(
                type A = x! { "a"*? "b"+? "c"^[1..3?] };
            )),
        );
    }
}
