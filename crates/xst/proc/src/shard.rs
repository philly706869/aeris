use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
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

    use ast::rust_expr::Expr;

    fn expand_magics(magics: &mut Vec<TokenStream>, expr: &Expr) {
        let mut stack = vec![expr];
        while let Some(item) = stack.pop() {
            match item {
                Expr::X(item) => {
                    magics.push(item.x_token.to_token_stream());
                }
                Expr::XBox(item) => {
                    magics.push(item.xbox_token.to_token_stream());
                    stack.push(&item.expr);
                }
                Expr::XOpt(item) => {
                    magics.push(item.xopt_token.to_token_stream());
                    stack.push(&item.expr);
                }
                Expr::XVec(item) => {
                    magics.push(item.xvec_token.to_token_stream());
                    stack.push(&item.expr);
                }
                Expr::XLOpt(item) => {
                    magics.push(item.xlopt_token.to_token_stream());
                    stack.push(&item.expr);
                }
                Expr::XLVec(item) => {
                    magics.push(item.xlvec_token.to_token_stream());
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
            magics.push(item.x_expr.x_token.to_token_stream());
        }
    }

    quote! {
        const _: () = {
            #(::xst::internal::#magics!();)*
        };
    }
}
