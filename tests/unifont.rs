use bdf_reader::Font;
use std::{fs::File, io::BufReader};

#[test]
fn parse_unifont() {
	let reader = BufReader::new(File::open("tests/unifont-15.0.01.bdf").unwrap());
	Font::read(reader).expect("Failed to parse font");
}
