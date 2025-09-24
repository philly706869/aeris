mod token;

use icu_properties::{
    CodePointSetData, CodePointSetDataBorrowed,
    props::{IdContinue, IdStart},
};
use once_cell::sync::Lazy;
pub use token::{Token, TokenModifier, TokenType};

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
// /// - [Mathematical Compatibility Notation Profile](https://www.unicode.org/reports/tr31/tr31-43.html#Mathematical_Compatibility_Notation_Profile)
// /// - [Emoji Profile](https://www.unicode.org/reports/tr31/tr31-43.html#Emoji_Profile)
// /// - [Default-Ignorable Exclusion Profile](https://www.unicode.org/reports/tr31/tr31-43.html#Default_Ignorable_Exclusion_Profile)
///
/// **Language-specific Extensions**
/// - Add U+0024 DOLLAR SIGN ($) to Start and Continue
/// - Add U+005F LOW LINE (_) to Start and Continue
pub struct IdentifierSyntax;

static ID_START_SET: Lazy<CodePointSetDataBorrowed<'static>> =
    Lazy::new(CodePointSetData::new::<IdStart>);
static ID_CONTINUE_SET: Lazy<CodePointSetDataBorrowed<'static>> =
    Lazy::new(CodePointSetData::new::<IdContinue>);
// static ID_COMPAT_MATH_START_SET: Lazy<CodePointSetDataBorrowed<'static>> =
//     Lazy::new(CodePointSetData::new::<IdCompatMathStart>);
// static ID_COMPAT_MATH_CONTINUE_SET: Lazy<CodePointSetDataBorrowed<'static>> =
//     Lazy::new(CodePointSetData::new::<IdCompatMathContinue>);

impl IdentifierSyntax {
    fn match_start(ch: char) -> bool {
        ID_START_SET.contains(ch) || matches!(ch, '$' | '_')
    }

    fn match_continue(ch: char) -> bool {
        ID_CONTINUE_SET.contains(ch)
    }
}

impl<'i> Syntax<'i> for IdentifierSyntax {
    type Output = &'i str;
    type Error = &'static str;

    fn parse(&self, input: &'i str) -> Result<Self::Output, Self::Error> {
        let mut n: usize = 0;
        let mut iter = input.chars();
        if !iter.next().is_some_and(Self::match_start) {
            return Err("");
        }
        while iter.next().is_some_and(Self::match_continue) {
            n += 1;
        }
        Ok(&input[..n])
    }
}
