mod token;

pub use token::{Token, TokenModifier, TokenType};

use unicode_general_category::{GeneralCategory as UC, get_general_category};

pub trait Syntax<'i> {
    type Output;
    type Error;

    fn parse(&self, input: &'i str) -> Result<Self::Output, Self::Error>;
}

/// Identifier syntax conforms to [UAX #31](https://www.unicode.org/reports/tr31/tr31-43.html),
/// using [R1-2](https://www.unicode.org/reports/tr31/tr31-43.html#R1-2) with the following profile:
///
/// **Base Definition**
/// - Start characters = `ID_Start`
/// - Continue characters = `ID_Continue`
/// - Normalization form = NFC
///
/// **Adopted Standard Profiles**
/// - [Mathematical Compatibility Notation Profile](https://www.unicode.org/reports/tr31/tr31-43.html#Mathematical_Compatibility_Notation_Profile)
/// - [Emoji Profile](https://www.unicode.org/reports/tr31/tr31-43.html#Emoji_Profile)
/// - [Default-Ignorable Exclusion Profile](https://www.unicode.org/reports/tr31/tr31-43.html#Default_Ignorable_Exclusion_Profile)
///
/// **Language-specific Extensions**
/// - Add U+0024 DOLLAR SIGN ($) to Start and Continue
/// - Add U+005F LOW LINE (_) to Start and Continue
pub struct IdentifierSyntax;

impl IdentifierSyntax {
    fn match_id_start(chr: char) -> bool {
        let category = get_general_category(chr);
        matches!(
            category,
            UC::UppercaseLetter | // Lu
            UC::LowercaseLetter | // Ll
            UC::TitlecaseLetter | // Lt
            UC::ModifierLetter | // Lm
            UC::OtherLetter | // Lo
            UC::LetterNumber // Nl
        ) || Self::match_other_id_start(chr)
            || Self::match_id_compat_math_start(chr)
    }

    fn match_id_continue(chr: char) -> bool {
        let category = get_general_category(chr);
        Self::match_id_start(chr)
            || matches!(
                category,
                UC::NonspacingMark | // Mn
                UC::SpacingMark | // Mc
                UC::DecimalNumber | // Nd 
                UC::ConnectorPunctuation // Pc
            )
            || Self::match_other_id_continue(chr)
            || Self::match_id_compat_math_continue(chr)
    }

    // fn match_xid_start(chr: char) -> bool {
    //     let category = get_general_category(chr);
    //     todo!()
    // }

    // fn match_xid_continue(chr: char) -> bool {
    //     let category = get_general_category(chr);
    //     todo!()
    // }

    fn match_other_id_start(chr: char) -> bool {
        matches!(chr, '\u{2118}' | '\u{212E}' | '\u{309B}' | '\u{309C}')
    }

    fn match_other_id_continue(chr: char) -> bool {
        matches!(chr, '\u{1369}' | '\u{00B7}' | '\u{0387}' | '\u{19DA}')
    }

    fn match_id_compat_math_start(chr: char) -> bool {
        todo!()
    }

    fn match_id_compat_math_continue(chr: char) -> bool {
        todo!()
    }

    fn match_pattern_syntax(chr: char) -> bool {
        let category = get_general_category(chr);
        todo!()
    }

    fn match_pattern_white_space(chr: char) -> bool {
        let category = get_general_category(chr);
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
