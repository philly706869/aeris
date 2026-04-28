use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::all_consuming,
    multi::{many0, many1},
    sequence::{preceded, separated_pair, terminated},
};

pub fn parse_syntax(input: &str) -> IResult<&str, Vec<Entry<'_>>> {
    all_consuming(many0(terminated(
        alt((
            preceded(tag("pub "), ident.map(Entry::Pub)),
            separated_pair(
                ident,
                tag(" ->"),
                many1(preceded(tag(" "), ident.map(RuleTree::NonTerminal))),
            )
            .map(|(name, items)| Entry::Rule(name, items)),
        )),
        tag("\n"),
    )))
    .parse(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
    take_while(|ch: char| ch.is_ascii_alphanumeric() || ch == '_').parse(input)
}

pub enum Entry<'str> {
    Pub(&'str str),
    Rule(&'str str, Vec<RuleTree<'str>>),
}

pub enum RuleTree<'str> {
    Terminal(&'str str),
    NonTerminal(&'str str),
}
