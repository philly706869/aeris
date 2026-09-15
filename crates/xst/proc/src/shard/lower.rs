use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitInt};

use crate::{
    names::Names,
    shard::ast::{self, prim_expr as prim, rust_expr as rust},
    shard::ir,
};

pub fn lower(shard: ast::Shard) -> syn::Result<ir::Shard> {
    let core_ident = names.fresh("shard_core");
    let marker_ident = names.fresh("marker");
    let (vis, ident, params) = match &shard {
        ast::Shard::Struct(s) => (&s.vis, &s.ident, &s.params),
        ast::Shard::Enum(s) => (&s.vis, &s.ident, &s.params),
        ast::Shard::Type(s) => (&s.vis, &s.ident, &s.params),
    };
    let mut context = Context {
        ident: ident.clone(),
        params: params.idents.iter().cloned().collect(),
        closures: Vec::new(),
        names,
    };
    for (index, param) in context.params.iter().enumerate() {
        if context.params[..index].contains(param) {
            return Err(syn::Error::new(param.span(), "duplicate shard parameter"));
        }
    }
    let vis = vis.clone();
    let (kind, data) = match shard {
        ast::Shard::Struct(s) => {
            let mut fields = Vec::new();
            let mut data = Vec::new();
            for field in s.fields {
                let expr = context.expr(&field.expr)?;
                fields.push((field.ident, expr.output));
                data.push(expr.data);
            }
            (ir::Kind::Struct(fields), sequence(data))
        }
        ast::Shard::Enum(s) => {
            let mut variants = Vec::new();
            let mut data = Vec::new();
            for variant in s.variants {
                if variants.iter().any(|(ident, _)| ident == &variant.ident) {
                    return Err(syn::Error::new(
                        variant.ident.span(),
                        "duplicate shard variant",
                    ));
                }
                let (public, private) = context.shard(&variant.shard_expr)?;
                variants.push((
                    variant.ident,
                    quote!(::xst::internal::ShardField<'i, #public>),
                ));
                data.push(reference(private));
            }
            (ir::Kind::Enum(variants), alternative(data))
        }
        ast::Shard::Type(s) => (ir::Kind::Type, context.sequence(&s.x_expr.sequence)?),
    };
    Ok(ir::Shard {
        vis,
        core_ident,
        marker_ident,
        ident: context.ident,
        params: context.params,
        kind,
        data,
        closures: context.closures,
    })
}

struct Context {
    names: Names,
    ident: Ident,
    params: Vec<Ident>,
    closures: Vec<ir::Closure>,
}

struct Expr {
    output: TokenStream,
    data: TokenStream,
}

impl Context {
    fn expr(&mut self, expr: &rust::Expr) -> syn::Result<Expr> {
        Ok(match expr {
            rust::Expr::X(expr) => Expr {
                output: quote!(&'i ::xst::internal::str),
                data: self.sequence(&expr.sequence)?,
            },
            rust::Expr::XBox(expr) => {
                let Expr { output, data } = self.expr(&expr.expr)?;
                Expr {
                    output: quote!(::xst::internal::Box<#output>),
                    data,
                }
            }
            rust::Expr::XOpt(expr) => self.option(&expr.expr, false)?,
            rust::Expr::XOptZ(expr) => self.option(&expr.expr, true)?,
            rust::Expr::XVec(expr) => {
                let bounds = match &expr.limit {
                    rust::Limit::Exact(limit) => exact(&limit.count),
                    rust::Limit::Range(limit) => range(&limit.start, &limit.end)?,
                };
                self.vec(&expr.expr, bounds, false)?
            }
            rust::Expr::XVecZ(expr) => {
                let bounds = range(&expr.limit.start, &expr.limit.end)?;
                self.vec(&expr.expr, bounds, true)?
            }
            rust::Expr::Tuple(expr) => {
                let mut outputs = Vec::new();
                let mut data = Vec::new();
                for expr in &expr.exprs {
                    let expr = self.expr(expr)?;
                    outputs.push(expr.output);
                    data.push(expr.data);
                }
                Expr {
                    output: quote!((#(#outputs,)*)),
                    data: sequence(data),
                }
            }
            rust::Expr::Shard(expr) => {
                let (public, private) = self.shard(expr)?;
                Expr {
                    output: quote!(::xst::internal::ShardField<'i, #public>),
                    data: reference(private),
                }
            }
        })
    }

    fn option(&mut self, expr: &rust::Expr, lazy: bool) -> syn::Result<Expr> {
        let Expr { output, data } = self.expr(expr)?;
        let constructor = if lazy {
            quote!(option_lazy)
        } else {
            quote!(option)
        };
        Ok(Expr {
            output: quote!(::xst::internal::Option<#output>),
            data: quote!(::xst::internal::ShardData::#constructor(&#data)),
        })
    }

    fn vec(
        &mut self,
        expr: &rust::Expr,
        (min, max): (TokenStream, TokenStream),
        lazy: bool,
    ) -> syn::Result<Expr> {
        let Expr { output, data } = self.expr(expr)?;
        Ok(Expr {
            output: quote!(::xst::internal::Vec<#output>),
            data: repetition(data, min, max, lazy),
        })
    }

    fn shard(&mut self, expr: &rust::ShardExpr) -> syn::Result<(TokenStream, TokenStream)> {
        let mut public = Vec::new();
        let mut private = Vec::new();
        for arg in &expr.args.exprs {
            let (a, b) = if let rust::Expr::Shard(shard) = arg {
                self.shard(shard)?
            } else {
                // Reserve the index before lowering nested arguments.
                let index = self.reserve_closure();
                let Expr { output, data } = self.expr(arg)?;
                self.closures[index].output = output;
                self.closures[index].data = data;
                self.closure_types(index)
            };
            public.push(a);
            private.push(b);
        }
        self.shard_types(&expr.ident, public, private)
    }

    fn shard_types(
        &self,
        ident: &Ident,
        public: Vec<TokenStream>,
        private: Vec<TokenStream>,
    ) -> syn::Result<(TokenStream, TokenStream)> {
        if self.params.contains(ident) {
            if !public.is_empty() {
                return Err(syn::Error::new(
                    ident.span(),
                    "shard parameters cannot take arguments",
                ));
            }
            Ok((quote!(#ident), quote!(#ident)))
        } else {
            Ok((
                quote!(#ident<'static, #(#public),*>),
                quote!(#ident<'static, #(#private),*>),
            ))
        }
    }

    fn reserve_closure(&mut self) -> usize {
        let index = self.closures.len();
        self.closures.push(ir::Closure {
            index,
            ident: self.names.fresh("shard_closure"),
            output: TokenStream::new(),
            data: TokenStream::new(),
        });
        index
    }

    fn closure_types(&self, index: usize) -> (TokenStream, TokenStream) {
        let ident = &self.ident;
        let params = &self.params;
        let closure = &self.closures[index].ident;
        let args = if params.is_empty() {
            quote!()
        } else {
            quote!(<#(#params),*>)
        };
        let index = syn::Index::from(index);
        (
            quote!(::xst::internal::ShardClosure<#ident<'static, #(#params),*>, #index>),
            quote!(#closure #args),
        )
    }

    fn sequence(&mut self, seq: &prim::Sequence) -> syn::Result<TokenStream> {
        seq.exprs
            .iter()
            .map(|expr| self.prim(expr))
            .collect::<syn::Result<Vec<_>>>()
            .map(sequence)
    }

    fn prim_shard(&mut self, shard: &prim::ShardAtom) -> syn::Result<TokenStream> {
        let mut public = Vec::new();
        let mut private = Vec::new();
        for arg in &shard.args.exprs {
            let ty = if let (prim::Atom::Shard(shard), prim::Modifier::None) =
                (&arg.atom, &arg.modifier)
            {
                self.prim_shard(shard)?
            } else {
                let index = self.reserve_closure();
                let data = self.prim(arg)?;
                self.closures[index].output = quote!(&'i ::xst::internal::str);
                self.closures[index].data = data;
                self.closure_types(index).1
            };
            public.push(ty.clone());
            private.push(ty);
        }
        Ok(self.shard_types(&shard.ident, public, private)?.1)
    }

    fn prim(&mut self, expr: &prim::Expr) -> syn::Result<TokenStream> {
        let data = match &expr.atom {
            prim::Atom::Lit(lit) => {
                let text = &lit.text;
                quote!(::xst::internal::ShardData::literal(#text))
            }
            prim::Atom::Set(set) => {
                let negated = set.bang_token.is_some();
                let mut ranges = Vec::new();
                for entry in &set.entries {
                    let start = &entry.start;
                    let end = entry.end.as_ref().unwrap_or(start);
                    if start.value() > end.value() {
                        return Err(syn::Error::new(
                            start.span(),
                            "character range start exceeds end",
                        ));
                    }
                    ranges.push(quote!(#start ..= #end));
                }
                quote!(::xst::internal::ShardData::set(#negated, &[#(#ranges),*]))
            }
            prim::Atom::Seq(seq) => self.sequence(&seq.sequence)?,
            prim::Atom::Alt(alt) => alternative(
                alt.entries
                    .iter()
                    .map(|entry| self.sequence(&entry.sequence))
                    .collect::<syn::Result<_>>()?,
            ),
            prim::Atom::Shard(shard) => reference(self.prim_shard(shard)?),
        };
        let (bounds, lazy) = match &expr.modifier {
            prim::Modifier::None => return Ok(data),
            prim::Modifier::Star { question_token, .. } => (
                (quote!(0), quote!(::xst::internal::Option::None)),
                question_token.is_some(),
            ),
            prim::Modifier::Plus { question_token, .. } => (
                (quote!(1), quote!(::xst::internal::Option::None)),
                question_token.is_some(),
            ),
            prim::Modifier::Caret {
                limit,
                question_token,
                ..
            } => match limit {
                prim::Limit::Exact(limit) => (exact(&limit.count), false),
                prim::Limit::Range(limit) => {
                    (range(&limit.start, &limit.end)?, question_token.is_some())
                }
            },
        };
        Ok(repetition(data, bounds.0, bounds.1, lazy))
    }
}

fn repetition(data: TokenStream, min: TokenStream, max: TokenStream, lazy: bool) -> TokenStream {
    let constructor = if lazy { quote!(vec_lazy) } else { quote!(vec) };
    quote!(::xst::internal::ShardData::#constructor(&#data, #min, #max))
}

fn reference(ty: TokenStream) -> TokenStream {
    quote!(::xst::internal::ShardData::reference::<#ty>())
}

fn sequence(mut data: Vec<TokenStream>) -> TokenStream {
    if data.len() == 1 {
        data.pop().unwrap()
    } else {
        quote!(::xst::internal::ShardData::sequence(&[#(&#data),*]))
    }
}

fn alternative(data: Vec<TokenStream>) -> TokenStream {
    quote!(::xst::internal::ShardData::alternative(&[#(&#data),*]))
}

fn exact(count: &LitInt) -> (TokenStream, TokenStream) {
    (
        quote!(#count),
        quote!(::xst::internal::Option::Some(#count)),
    )
}

fn range(start: &Option<LitInt>, end: &Option<LitInt>) -> syn::Result<(TokenStream, TokenStream)> {
    if let Some(end) = end {
        let min = start
            .as_ref()
            .map(|start| start.base10_parse::<u128>())
            .transpose()?
            .unwrap_or(0);
        if min > end.base10_parse::<u128>()? {
            return Err(syn::Error::new(
                end.span(),
                "repetition range start exceeds end",
            ));
        }
    }
    let min = start
        .as_ref()
        .map_or_else(|| quote!(0), |start| quote!(#start));
    let max = end.as_ref().map_or_else(
        || quote!(::xst::internal::Option::None),
        |end| quote!(::xst::internal::Option::Some(#end)),
    );
    Ok((min, max))
}
