use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

mod ast;

pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    let expanded_attr = expand_attr(attr);
    let expanded_item = expand_item(item);
    let mut expanded = TokenStream::new();
    expanded.extend(expanded_item);
    expanded.extend(expanded_attr);
    expanded
}

fn expand_attr(attr: TokenStream) -> TokenStream {
    if attr.is_empty() {
        return TokenStream::new();
    }
    syn::Error::new_spanned(attr, "unexpected attribute argument").into_compile_error()
}

fn expand_item(item: TokenStream) -> TokenStream {
    let ast: ast::Shard = match parse2(item) {
        Ok(ast) => ast,
        Err(err) => return err.into_compile_error(),
    };

    use ast::rust_expr::{Expr, keyword};

    enum Magic {
        X(keyword::x),
        XBox(keyword::xbox),
        XOpt(keyword::xopt),
        XVec(keyword::xvec),
        XLOpt(keyword::xlopt),
        XLVec(keyword::xlvec),
    }

    fn expand_magics(magics: &mut Vec<Magic>, expr: &Expr) {
        let mut stack = vec![expr];
        while let Some(item) = stack.pop() {
            match item {
                Expr::X(item) => {
                    magics.push(Magic::X(item.x_token));
                }
                Expr::XBox(item) => {
                    magics.push(Magic::XBox(item.xbox_token));
                    stack.push(&item.expr);
                }
                Expr::XOpt(item) => {
                    magics.push(Magic::XOpt(item.xopt_token));
                    stack.push(&item.expr);
                }
                Expr::XVec(item) => {
                    magics.push(Magic::XVec(item.xvec_token));
                    stack.push(&item.expr);
                }
                Expr::XLOpt(item) => {
                    magics.push(Magic::XLOpt(item.xlopt_token));
                    stack.push(&item.expr);
                }
                Expr::XLVec(item) => {
                    magics.push(Magic::XLVec(item.xlvec_token));
                    stack.push(&item.expr);
                }
                Expr::Tuple(item) => {
                    stack.extend(item.exprs.iter());
                }
                Expr::Shard(item) => {
                    stack.extend(item.args.exprs.iter());
                }
            }
        }
    }

    let mut magics = Vec::new();

    match ast {
        ast::Shard::Struct(item) => {
            for field in item.fields {
                expand_magics(&mut magics, &field.expr);
            }
        }
        ast::Shard::Enum(item) => {
            for variant in item.variants.iter() {
                for expr in variant.shard_expr.args.exprs.iter() {
                    expand_magics(&mut magics, expr);
                }
            }
        }
        ast::Shard::Type(item) => {
            magics.push(Magic::X(item.x_expr.x_token));
        }
    }

    let magics = magics.iter().map(|magic| match magic {
        Magic::X(token) => quote! { #token! {} },
        Magic::XBox(token) => quote! { #token! []; },
        Magic::XOpt(token) => quote! { #token! []; },
        Magic::XVec(token) => quote! { #token! []; },
        Magic::XLOpt(token) => quote! { #token! []; },
        Magic::XLVec(token) => quote! { #token! []; },
    });

    quote! {
        const _: () = {
            /// # XST x!
            macro_rules! x {
                {} => {}
            }
            /// # XST xbox!
            macro_rules! xbox {
                [] => {}
            }
            /// # XST xopt!
            macro_rules! xopt {
                [] => {}
            }
            /// # XST xvec!
            macro_rules! xvec {
                [] => {}
            }
            /// # XST xlopt!
            macro_rules! xlopt {
                [] => {}
            }
            /// # XST xlvec!
            macro_rules! xlvec {
                [] => {}
            }
            #(#magics)*
        };
    }
}
