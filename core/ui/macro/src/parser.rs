use std::collections::HashSet;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    multi::{many0, many1},
    sequence::{pair, preceded, separated_pair, terminated},
};

pub fn parse_syntax(input: &str) -> IResult<&str, ()> {
    let pubs: HashSet<String> = HashSet::new();

    many0(terminated(
        alt((
            preceded(tag("pub "), parse_ident.map(|name| Entry::Pub(name))),
            separated_pair(
                parse_ident,
                tag(" ->"),
                many1(preceded(tag(" "), parse_ident)),
            )
            .map(|(name, items)| Entry::Rule(name, todo!())),
        )),
        tag("\n"),
    ))
    .parse(input)?;

    todo!();
}

fn parse_ident(input: &str) -> IResult<&str, &str> {
    take_while(|ch: char| ch.is_ascii_alphanumeric() || ch == '_').parse(input)
}

enum Entry<'str> {
    Pub(&'str str),
    Rule(&'str str, Vec<()>),
}
