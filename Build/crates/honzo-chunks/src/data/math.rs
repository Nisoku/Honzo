use std::str;
use std::string::String;

use quick_xml::Reader;

use honzo_core::{HonzoError, MathType};
use pulldown_latex::{push_mathml, Parser as LatexParser, RenderConfig, Storage as LatexStorage};

pub const MATH_TAG: [u8; 4] = *b"MATH";

pub fn validate_mathml(bytes: &[u8]) -> Result<(), HonzoError> {
    let mut reader = Reader::from_reader(bytes);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(_) => buf.clear(),
            Err(_) => return Err(HonzoError::InvalidMathML),
        }
    }
    Ok(())
}

pub fn latex_to_mathml(bytes: &[u8]) -> Result<String, HonzoError> {
    let s = match str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_) => return Err(HonzoError::Truncated),
    };

    // Use pulldown-latex to render LaTeX math to MathML.
    let storage = LatexStorage::new();
    let parser = LatexParser::new(s, &storage);
    let mut out = String::new();
    let cfg = RenderConfig {
        xml: true,
        ..Default::default()
    };
    push_mathml(&mut out, parser, cfg).map_err(|_| HonzoError::InvalidMathML)?;
    Ok(out)
}

pub fn render_math(bytes: &[u8], math_type: MathType) -> Result<String, HonzoError> {
    match math_type {
        MathType::MathML => {
            validate_mathml(bytes)?;
            let s = str::from_utf8(bytes).map_err(|_| HonzoError::Truncated)?;
            Ok(s.to_string())
        }
        MathType::LaTeX => latex_to_mathml(bytes),
    }
}
