mod token;

pub use token::{Token, TokenModifier, TokenType};

use unicode_general_category::{GeneralCategory as UC, get_general_category};

pub trait Syntax<'i> {
    type Output;
    type Error;

    fn parse(&self, input: &'i str) -> Result<Self::Output, Self::Error>;
}

/// Identifier syntax based on [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/tr31-43.html)
pub struct IdentifierSyntax;

impl IdentifierSyntax {
    fn match_id_start(chr: char) -> bool {
        let category = get_general_category(chr);
        matches!(
            category,
            UC::UppercaseLetter
                | UC::LowercaseLetter
                | UC::TitlecaseLetter
                | UC::ModifierLetter
                | UC::OtherLetter
                | UC::LetterNumber
        )
    }

    fn match_id_continue(chr: char) -> bool {
        let category = get_general_category(chr);
        Self::match_id_start(chr) || matches!(category, UC::UppercaseLetter)
    }

    fn match_xid_start(chr: char) -> bool {
        let category = get_general_category(chr);
        todo!()
    }

    fn match_xid_continue(chr: char) -> bool {
        let category = get_general_category(chr);
        todo!()
    }

    fn match_other_id_start(chr: char) -> bool {
        todo!()
    }

    fn match_other_id_continue(chr: char) -> bool {
        todo!()
    }

    fn match_pattern_syntax(chr: char) -> bool {
        todo!()
    }

    fn match_pattern_white_space(chr: char) -> bool {
        todo!()
    }
}

impl<'i> Syntax<'i> for IdentifierSyntax {
    type Output = &'i str;
    type Error = &'static str;

    fn parse(&self, input: &'i str) -> Result<Self::Output, Self::Error> {
        let mut n: usize = 0;
        let mut iter = input.chars();
        if !iter.next().is_some_and(Self::match_id_start) {
            return Err("");
        }
        while iter.next().is_some_and(Self::match_id_continue) {
            n += 1;
        }
        Ok(&input[..n])
    }
}
