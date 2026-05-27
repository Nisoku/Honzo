use cssparser::{
    AtRuleParser, ParseError, Parser, ParserInput, QualifiedRuleParser, StyleSheetParser,
};
use honzo_core::HonzoError;

pub const CSS_TAG: [u8; 4] = *b"CSS_";

pub fn is_css_tag(tag: &[u8; 4]) -> bool {
    *tag == CSS_TAG
}

struct CssValidator;

fn parse_entirely<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, ()>> {
    while !input.is_exhausted() {
        input.next()?;
    }
    Ok(())
}

impl<'i> QualifiedRuleParser<'i> for CssValidator {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        input.parse_entirely(parse_entirely)
    }

    fn parse_block<'t>(
        &mut self,
        _prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        input.parse_entirely(parse_entirely)
    }
}

impl<'i> AtRuleParser<'i> for CssValidator {
    type Prelude = ();
    type AtRule = ();
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        _name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        input.parse_entirely(parse_entirely)
    }

    fn parse_block<'t>(
        &mut self,
        _prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        input.parse_entirely(parse_entirely)
    }

    fn rule_without_block<'t>(
        &mut self,
        _prelude: Self::Prelude,
        _start: &cssparser::ParserState,
    ) -> Result<Self::AtRule, ()> {
        Ok(())
    }
}

/// Validate CSS bytes by parsing as a stylesheet with cssparser.
/// Returns the CSS as a string slice if valid.
pub fn validate_css(bytes: &[u8]) -> Result<&str, HonzoError> {
    let s = core::str::from_utf8(bytes).map_err(|_| HonzoError::Truncated)?;
    let mut input = ParserInput::new(s);
    let mut parser = Parser::new(&mut input);
    let mut validator = CssValidator;
    let mut list_parser = StyleSheetParser::new(&mut parser, &mut validator);
    loop {
        match list_parser.next() {
            Some(Ok(..)) => continue,
            Some(Err((ParseError { .. }, _))) => return Err(HonzoError::InvalidCss),
            None => break,
        }
    }
    Ok(s)
}

pub fn validate_css_bytes(bytes: &[u8]) -> Result<(), u8> {
    match validate_css(bytes) {
        Ok(..) => Ok(()),
        Err(HonzoError::Truncated) => Err(7),
        Err(HonzoError::InvalidCss) => Err(8),
        Err(..) => Err(255),
    }
}

pub fn chunk_name() -> &'static str {
    "stylesheet"
}
