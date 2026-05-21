use honzo_core::HonzoParser;
use honzo_core::{Compression, MathType};
use honzo_io::{HonzoBuilder, HonzoStream};
use std::io::Cursor;

#[test]
fn math_chunk_roundtrip_mathml() {
    let math = b"<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>x</mi></math>";

    let file = HonzoBuilder::new()
        .add_math_chunk(math, MathType::MathML, Compression::None)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    let entry = parser.find_chunk(b"MATH").expect("MATH chunk present");

    assert_eq!(entry.content_type_kind, 2);
    assert_eq!(entry.content_type_value, MathType::MathML as u8);

    let mut stream = HonzoStream::open(Cursor::new(&file), 1).unwrap();
    let got = stream.read_chunk(&entry).unwrap();
    assert_eq!(got, math);
}
