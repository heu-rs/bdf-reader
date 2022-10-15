use bdf_reader::Font;
use simple_logger::SimpleLogger;
use std::{fs::File, io::BufReader};

#[test]
fn parse_unifont() {
	_ = SimpleLogger::new().init();

	let reader = BufReader::new(File::open("tests/unifont-15.0.01.bdf").unwrap());
	Font::read(reader).expect("Failed to parse font");
}
