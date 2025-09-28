use aeris::syntax::{Context, Syntax};
use icu_properties::{
    CodePointSetData,
    props::{IdContinue, IdStart},
};

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
pub struct Identifier;

impl Identifier {
    fn match_start(ch: char) -> bool {
        let id_start_set = CodePointSetData::new::<IdStart>();
        id_start_set.contains(ch) || matches!(ch, '$' | '_')
    }

    fn match_continue(ch: char) -> bool {
        let id_continue_set = CodePointSetData::new::<IdContinue>();
        id_continue_set.contains(ch)
    }
}

// impl<'i> Syntax<'i> for Identifier {
//     type Output = Result<&'i str, String>;

//     fn parse(&self, ctx: Context<'i>) -> Self::Output {
//         let span = ctx.span();
//         let mut chars = ctx.input().chars();
//         if !chars.next().is_some_and(Self::match_start) {
//             return Err("".to_owned());
//         }
//         chars.take_while(|ch| Self::match_continue(*ch));
//         Ok(todo!())
//     }
// }
