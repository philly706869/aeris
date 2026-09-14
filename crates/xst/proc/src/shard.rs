use proc_macro2::TokenStream;

mod ast;
mod emit;
mod ir;
mod lower;

use crate::names::Names;

pub fn expand(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(attr, "unexpected argument"));
    }
    let shard: ast::Shard = syn::parse2(item.clone())?;
    let names = Names::new(item);
    let ir = lower::lower(shard, names)?;
    Ok(emit::emit(ir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    fn expand(input: TokenStream) -> syn::Result<TokenStream> {
        super::expand(TokenStream::new(), input)
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
    fn rejects_lazy_marker_inside_bounds_or_after_exact_count() {
        for input in [
            quote!(
                type A = x! { "a"^[..?] };
            ),
            quote!(
                type A = x! { "a"^[10..?] };
            ),
            quote!(
                type A = x! { "a"^[..10?] };
            ),
            quote!(
                type A = x! { "a"^[10..20?] };
            ),
            quote!(
                type A = x! { "a"^[10]? };
            ),
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
    fn z_and_question_modifiers_generate_lazy_data() {
        fn data(input: TokenStream) -> String {
            let ast = syn::parse2(input.clone()).unwrap();
            let names = Names::new(input);
            let ir = lower::lower(ast, names).unwrap();
            ir.data.to_string()
        }
        for (greedy, lazy) in [
            (
                quote!(
                    struct A {
                        a: xopt![B],
                        b: xvec![B, 1..3],
                    }
                ),
                quote!(
                    struct A {
                        a: xoptz![B],
                        b: xvecz![B, 1..3],
                    }
                ),
            ),
            (
                quote!(
                    type A = x! { "a"* "b"+ "c"^[1..3] "d"^[..]? };
                ),
                quote!(
                    type A = x! { "a"*? "b"+? "c"^[1..3]? "d"^[..]? };
                ),
            ),
        ] {
            let greedy = data(greedy);
            let lazy = data(lazy);
            assert_ne!(greedy, lazy);
            assert_eq!(greedy.replace("_lazy", ""), lazy.replace("_lazy", ""));
        }
    }
}
