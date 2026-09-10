//! Syntax trees for shard declarations and their two expression syntaxes.
use proc_macro2::Span;
use syn::{
    Ident, LitChar, LitInt, LitStr, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Bracket, Paren},
};

#[derive(Debug)]
pub struct Shard {
    pub visibility: Visibility,
    pub name: Ident,
    pub parameters: Vec<Ident>,
    pub binding: Binding,
}

#[derive(Debug)]
pub enum Binding {
    Struct(Vec<Field>),
    Enum(Vec<Variant>),
    Forward(MagicSequence),
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct Variant {
    pub name: Ident,
    pub shard: Reference<Expression>,
}

#[derive(Debug)]
pub struct Reference<T> {
    pub name: Ident,
    pub arguments: Vec<T>,
}

#[derive(Debug)]
pub enum Expression {
    Shard(Reference<Expression>),
    Magic(MagicSequence),
    Box(Box<Expression>),
    Option(Box<Expression>),
    Vec {
        item: Box<Expression>,
        range: RepeatRange,
    },
    Tuple(Vec<Expression>),
}

/// Inclusive bounds are preserved as written; None is an open bound. A single integer
/// has equal lower and upper bounds. No usize::MAX sentinel is needed here.
#[derive(Debug)]
pub struct RepeatRange {
    pub min: Option<LitInt>,
    pub max: Option<LitInt>,
    pub exact: bool,
}

#[derive(Debug)]
pub struct MagicSequence(pub Vec<MagicExpression>);

#[derive(Debug)]
pub struct MagicExpression {
    pub atom: MagicAtom,
    pub quantifier: Option<Quantifier>,
}

#[derive(Debug)]
pub enum MagicAtom {
    Shard(Reference<MagicSequence>),
    String(LitStr),
    Set {
        negated: bool,
        items: Vec<CharacterRange>,
    },
    Tuple(MagicSequence),
    Alternative(Vec<MagicSequence>),
}

#[derive(Debug)]
pub struct CharacterRange {
    pub start: LitChar,
    pub end: Option<LitChar>,
}

#[derive(Debug)]
pub struct Quantifier {
    pub span: Span,
    pub kind: QuantifierKind,
    pub lazy: bool,
}

#[derive(Debug)]
pub enum QuantifierKind {
    OneOrMore,
    ZeroOrMore,
    Optional,
    Range(RepeatRange),
}

fn parse_all<T: Parse>(input: ParseStream) -> syn::Result<T> {
    let value = input.parse()?;
    if !input.is_empty() {
        return Err(input.error("unexpected trailing tokens"));
    }
    Ok(value)
}

fn parameters(input: ParseStream) -> syn::Result<Vec<Ident>> {
    if !input.peek(Token![<]) {
        return Ok(Vec::new());
    }
    input.parse::<Token![<]>()?;
    if input.peek(Token![>]) {
        return Err(input.error("expected generic parameter"));
    }
    let mut items = Vec::<Ident>::new();
    loop {
        let name: Ident = input.parse()?;
        if items.iter().any(|item| item == &name) {
            return Err(syn::Error::new(name.span(), "duplicate generic parameter"));
        }
        items.push(name);
        if !input.peek(Token![,]) {
            break;
        }
        input.parse::<Token![,]>()?;
        if input.peek(Token![>]) {
            break;
        }
    }
    input.parse::<Token![>]>()?;
    Ok(items)
}

impl Parse for Shard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let kind = if input.peek(Token![struct]) {
            input.parse::<Token![struct]>()?;
            0
        } else if input.peek(Token![enum]) {
            input.parse::<Token![enum]>()?;
            1
        } else if input.peek(Token![type]) {
            input.parse::<Token![type]>()?;
            2
        } else {
            return Err(input.error("expected struct, enum, or type shard"));
        };
        let name = input.parse()?;
        let parameters = parameters(input)?;
        let binding = match kind {
            0 => {
                let content;
                braced!(content in input);
                Binding::Struct(
                    Punctuated::<Field, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect(),
                )
            }
            1 => {
                let content;
                braced!(content in input);
                let variants = Punctuated::<Variant, Token![,]>::parse_terminated(&content)?;
                let mut names = std::collections::HashSet::new();
                for variant in &variants {
                    if !names.insert(variant.name.to_string()) {
                        return Err(syn::Error::new(variant.name.span(), "duplicate variant"));
                    }
                }
                Binding::Enum(variants.into_iter().collect())
            }
            _ => {
                input.parse::<Token![=]>()?;
                let magic: Ident = input.parse()?;
                if magic != "x" {
                    return Err(syn::Error::new(
                        magic.span(),
                        "forward shard requires x! { ... }",
                    ));
                }
                input.parse::<Token![!]>()?;
                let content;
                braced!(content in input);
                let expression = parse_all(&content)?;
                input.parse::<Token![;]>()?;
                Binding::Forward(expression)
            }
        };
        Ok(Self {
            visibility,
            name,
            parameters,
            binding,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            expression: {
                input.parse::<Token![:]>()?;
                input.parse()?
            },
        })
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let content;
        parenthesized!(content in input);
        let shard = content.parse()?;
        // A trailing comma is punctuation, not a second payload.
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
        if !content.is_empty() {
            return Err(content.error("expected exactly one shard reference"));
        }
        Ok(Self { name, shard })
    }
}

impl<T: Parse> Parse for Reference<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let mut arguments = Vec::new();
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            if input.peek(Token![>]) {
                return Err(input.error("expected generic argument"));
            }
            loop {
                arguments.push(input.parse()?);
                if !input.peek(Token![,]) {
                    break;
                }
                input.parse::<Token![,]>()?;
                if input.peek(Token![>]) {
                    break;
                }
            }
            input.parse::<Token![>]>()?;
        }
        Ok(Self { name, arguments })
    }
}

impl Parse for Expression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            return Ok(Self::Tuple(
                Punctuated::<Expression, Token![,]>::parse_terminated(&content)?
                    .into_iter()
                    .collect(),
            ));
        }
        let fork = input.fork();
        let name: Ident = fork.parse()?;
        if !fork.peek(Token![!]) {
            return Ok(Self::Shard(input.parse()?));
        }
        input.parse::<Ident>()?;
        input.parse::<Token![!]>()?;
        if name == "x" {
            let content;
            braced!(content in input);
            return Ok(Self::Magic(parse_all(&content)?));
        }
        if name != "xbox" && name != "xopt" && name != "xvec" {
            return Err(syn::Error::new(name.span(), "unknown shard macro"));
        }
        let content;
        bracketed!(content in input);
        let item = Box::new(content.parse()?);
        let result = if name == "xbox" {
            Self::Box(item)
        } else if name == "xopt" {
            Self::Option(item)
        } else {
            content.parse::<Token![|]>()?;
            Self::Vec {
                item,
                range: content.parse()?,
            }
        };
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in shard macro"));
        }
        Ok(result)
    }
}

impl Parse for RepeatRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let min = if input.peek(LitInt) {
            Some(input.parse::<LitInt>()?)
        } else {
            None
        };
        let (max, exact) = if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            let max = if input.peek(LitInt) {
                Some(input.parse::<LitInt>()?)
            } else {
                None
            };
            (max, false)
        } else if let Some(min) = &min {
            (Some(min.clone()), true)
        } else {
            return Err(input.error("expected repetition count or range"));
        };
        for bound in [&min, &max].into_iter().flatten() {
            if !bound.suffix().is_empty() {
                return Err(syn::Error::new(
                    bound.span(),
                    "repetition count must not have a suffix",
                ));
            }
            bound.base10_parse::<usize>()?;
        }
        if let (Some(min), Some(max)) = (&min, &max) {
            if min.base10_parse::<usize>()? > max.base10_parse::<usize>()? {
                return Err(syn::Error::new(
                    max.span(),
                    "maximum repetition count is below minimum",
                ));
            }
        }
        Ok(Self { min, max, exact })
    }
}

impl Parse for MagicSequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty()
            && !input.peek(Token![,])
            && !input.peek(Token![>])
            && !input.peek(Token![|])
        {
            items.push(input.parse()?);
        }
        if items.is_empty() && !input.is_empty() {
            return Err(input.error("expected magic expression"));
        }
        Ok(Self(items))
    }
}

impl Parse for MagicExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let atom = if input.peek(LitStr) {
            MagicAtom::String(input.parse()?)
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);
            let negated = if content.peek(Token![!]) {
                content.parse::<Token![!]>()?;
                true
            } else {
                false
            };
            let mut items = Vec::new();
            while !content.is_empty() {
                let start: LitChar = content.parse()?;
                let end = if content.peek(Token![..]) {
                    content.parse::<Token![..]>()?;
                    let end: LitChar = content.parse()?;
                    if start.value() > end.value() {
                        return Err(syn::Error::new(end.span(), "reversed character range"));
                    }
                    Some(end)
                } else {
                    None
                };
                items.push(CharacterRange { start, end });
            }
            MagicAtom::Set { negated, items }
        } else if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            MagicAtom::Tuple(parse_all(&content)?)
        } else if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            let mut alternatives = Vec::new();
            while !content.is_empty() {
                content.parse::<Token![|]>()?;
                let branch: MagicSequence = content.parse()?;
                if branch.0.is_empty() {
                    return Err(content.error("empty branch; use () for an empty sequence"));
                }
                alternatives.push(branch);
            }
            MagicAtom::Alternative(alternatives)
        } else {
            MagicAtom::Shard(input.parse()?)
        };
        let span = input.span();
        let kind = if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Some(QuantifierKind::OneOrMore)
        } else if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Some(QuantifierKind::ZeroOrMore)
        } else if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Some(QuantifierKind::Optional)
        } else if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            let content;
            bracketed!(content in input);
            Some(QuantifierKind::Range(parse_all(&content)?))
        } else {
            None
        };
        let quantifier = if let Some(kind) = kind {
            let lazy = if input.peek(Token![?]) {
                input.parse::<Token![?]>()?;
                true
            } else {
                false
            };
            Some(Quantifier { span, kind, lazy })
        } else {
            None
        };
        if input.peek(Token![+])
            || input.peek(Token![*])
            || input.peek(Token![?])
            || input.peek(Token![!])
        {
            return Err(input.error("directly nested quantifiers are not allowed; use parentheses"));
        }
        Ok(Self { atom, quantifier })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declarations_and_duplicate_fields() {
        let shard: Shard = syn::parse_str(
            r#"pub(crate) struct Braced<T> {
            brace: x! { "{" }, inner: T, brace: x! { "}" },
        }"#,
        )
        .unwrap();
        assert_eq!(shard.name, "Braced");
        assert!(matches!(shard.visibility, Visibility::Restricted(_)));
        assert_eq!(shard.parameters[0], "T");
        let Binding::Struct(fields) = shard.binding else {
            panic!()
        };
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, fields[2].name);
        for source in [
            "struct Empty {}",
            "enum Empty {}",
            "pub enum Value { A(AValue), B(BValue), C(CValue) }",
            "enum Foo<T> { T(T), Null(Null), Bar(Bar<T, Baz>) }",
            r#"type WithGeneric<T> = x! { "[" T "]" };"#,
            r#"pub type Digit = x! { {'0'..'9'} };"#,
        ] {
            syn::parse_str::<Shard>(source).unwrap_or_else(|e| panic!("{source}: {e}"));
        }
    }

    #[test]
    fn ordinary_expressions() {
        for source in [
            "Foo",
            r#"Bar<Foo, x! { "apple" }>"#,
            "Baz<(Foo, Name)>",
            "(Foo)",
            "(Foo,)",
            "(Foo, Bar)",
            "()",
            r#"(Foo, Bar, x! { "banana" })"#,
            "xbox![Foo]",
            "xopt![xbox![(Foo, Bar)]]",
            "xvec![Foo | ..]",
            "xvec![Foo | 10..]",
            "xvec![Foo | ..20]",
            "xvec![Foo | 10..20]",
            "xvec![Foo | 10]",
        ] {
            syn::parse_str::<Expression>(source).unwrap_or_else(|e| panic!("{source}: {e}"));
        }
    }

    #[test]
    fn magic_expressions() {
        for source in [
            "Foo",
            r#"Bar<Foo, "apple">"#,
            "Baz<(Foo Name)>",
            r#""" "apple" "Hello\nWorld""#,
            "{'a' 'c' 'g'}",
            "{'a'..'f' '0'}",
            r#"{! '\n'}"#,
            "{! 'a'..'z'}",
            "{}",
            "{!}",
            "(Foo Bar Baz)",
            r#"(Foo "apple"+)"#,
            "(Foo)",
            "()",
            r#"[| "apple" | "banana"]"#,
            "[| Foo | Bar | Baz<Foo, Bar>]",
            "[| Foo]",
            "[]",
            "(Foo+)+",
            "(Foo+)![10]",
            "Foo Bar<Baz<Qux>>",
        ] {
            syn::parse_str::<MagicSequence>(source).unwrap_or_else(|e| panic!("{source}: {e}"));
        }
        for quantifier in [
            "+",
            "*",
            "?",
            "![10]",
            "![10..]",
            "![..10]",
            "![10..20]",
            "![..]",
        ] {
            for suffix in ["", "?"] {
                let source = format!("Foo{quantifier}{suffix}");
                let parsed: MagicExpression = syn::parse_str(&source).unwrap();
                assert_eq!(parsed.quantifier.unwrap().lazy, suffix == "?");
            }
        }
    }

    #[test]
    fn ranges_preserve_inclusive_bounds() {
        for (source, min, max, exact) in [
            ("..", None, None, false),
            ("10..", Some(10), None, false),
            ("..20", None, Some(20), false),
            ("10..20", Some(10), Some(20), false),
            ("10", Some(10), Some(10), true),
            ("0..0", Some(0), Some(0), false),
        ] {
            let parsed: RepeatRange = syn::parse_str(source).unwrap();
            assert_eq!(parsed.min.map(|n| n.base10_parse::<usize>().unwrap()), min);
            assert_eq!(parsed.max.map(|n| n.base10_parse::<usize>().unwrap()), max);
            assert_eq!(parsed.exact, exact);
        }
    }

    #[test]
    fn invalid_declarations() {
        for source in [
            "struct Foo(Bar);",
            "struct Foo;",
            "struct Foo<T: Bar> {}",
            "struct Foo<T> where T: Bar {}",
            "struct Foo<T = Bar> {}",
            "struct Foo<'a> {}",
            "struct Foo<const N: usize> {}",
            "struct Foo<T,T> {}",
            "enum Foo { A {} }",
            "enum Foo { A }",
            "enum Foo { A() }",
            "enum Foo { A(Bar, Baz) }",
            "enum Foo { A(x! {}) }",
            "enum Foo { A(xopt![Bar]) }",
            "enum Foo { A((Bar)) }",
            "enum Foo { A(Bar), A(Baz) }",
            "type Foo = Bar;",
            "type Foo = x! [];",
            "type Foo = xopt![Bar];",
            "type Foo = x! {}",
            "struct Foo {} extra",
        ] {
            assert!(
                syn::parse_str::<Shard>(source).is_err(),
                "accepted {source}"
            );
        }
    }

    #[test]
    fn invalid_expressions() {
        for source in [
            "x![]",
            "xopt!{}",
            "xbox!(Foo)",
            "xvec![Foo; ..]",
            "xvec![Foo]",
            "xvec![Foo | 20..10]",
            "xvec![Foo | -1]",
            "xvec![Foo | 1usize]",
            "xvec![Foo | 1..=2]",
            "xvec![Foo | .. garbage]",
            "xopt![Foo Bar]",
            "(Foo Bar)",
            "Foo<>",
            "other![Foo]",
            "Foo::Bar",
            "x! { (Foo, Bar) }",
            "x! { Foo, Bar }",
            "x! { Foo<,> }",
            "x! { Foo![10 20] }",
            "x! { Foo![..=20] }",
        ] {
            assert!(
                syn::parse_str::<Expression>(source).is_err(),
                "accepted {source}"
            );
        }
        for source in [
            "Foo++",
            "Foo+*",
            "Foo+?![10]",
            "Foo???",
            "Foo![10]*",
            "[Foo]",
            "[|]",
            "[| Foo |]",
            "{'z'..'a'}",
            "{'a', 'b'}",
            "{'a'..='b'}",
        ] {
            assert!(
                syn::parse_str::<MagicSequence>(source).is_err(),
                "accepted {source}"
            );
        }
    }
}
