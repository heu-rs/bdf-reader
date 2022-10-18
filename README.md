# bdf-reader ![License: EPL-2.0](https://img.shields.io/badge/license-EPL--2.0-blue) [![bdf-reader on crates.io](https://img.shields.io/crates/v/bdf-reader)](https://crates.io/crates/bdf-reader) [![bdf-reader on docs.rs](https://docs.rs/bdf-reader/badge.svg)](https://docs.rs/bdf-reader) [![Source Code Repository](https://img.shields.io/badge/Code-On%20github.com-blue)](https://github.com/heu-rs/bdf-reader) [![bdf-reader on deps.rs](https://deps.rs/repo/github/heu-rs/bdf-reader/status.svg)](https://deps.rs/repo/github/heu-rs/bdf-reader)


## BDF Reader

A reader for the BDF ([Glyph Bitmap Distribution Format][__link0]) font format.


### Example


```rust
use bdf_reader::Font;
use std::{fs::File, io::BufReader};

let reader = BufReader::new(File::open("path/to/font.bdf")?);
let font = Font::read(reader)?;
```


 [__link0]: https://en.wikipedia.org/wiki/Glyph_Bitmap_Distribution_Format
