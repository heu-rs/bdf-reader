use crate::{
	tokens::{Token, WritingDirection},
	BoundingBox, Error, Font, Glyph, Size, Value
};
use std::{
	collections::{BTreeSet, HashMap},
	io::BufRead
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
	/// Initial state.
	Initial,

	/// Between start and end font.
	Font,

	/// Inside the font's properties, with `len` properties remaining.
	Properties { len: usize },

	/// Inside the font's chars, with `len` glyphs remaining.
	Chars { len: usize },

	/// Inside one of the font's chars, with `chars` glyphs remaining including the
	/// current one.
	Glyph { chars: usize },

	/// Final state
	Final
}

impl State {
	fn assert_initial(self, token: &Token) -> Result<(), Error> {
		match self {
			Self::Initial => Ok(()),
			_ => Err(Error::InvalidContext(token.clone(), self, "Initial"))
		}
	}

	fn assert_font(self, token: &Token) -> Result<(), Error> {
		match self {
			Self::Font => Ok(()),
			_ => Err(Error::InvalidContext(token.clone(), self, "Font"))
		}
	}

	fn assert_properties(self, token: &Token) -> Result<usize, Error> {
		match self {
			Self::Properties { len } => Ok(len),
			_ => Err(Error::InvalidContext(token.clone(), self, "Properties"))
		}
	}

	fn assert_chars(self, token: &Token) -> Result<usize, Error> {
		match self {
			Self::Chars { len } => Ok(len),
			_ => Err(Error::InvalidContext(token.clone(), self, "Chars"))
		}
	}

	fn assert_glyph(self, token: &Token) -> Result<usize, Error> {
		match self {
			Self::Glyph { chars } => Ok(chars),
			_ => Err(Error::InvalidContext(token.clone(), self, "Glyph"))
		}
	}
}

impl Font {
	pub fn read<R: BufRead>(reader: R) -> Result<Self, Error> {
		let mut font_version = None;
		let mut font_name = None;
		let mut font_size = None;
		let mut font_bbox = None;
		let mut font_swidth = None;
		let mut font_dwidth = None;
		let mut font_properties = HashMap::new();
		let mut font_glyphs = BTreeSet::new();

		let mut glyph_name = None;
		let mut glyph_encoding = None;
		let mut glyph_swidth = None;
		let mut glyph_dwidth = None;
		let mut glyph_bbox = None;

		let mut state = State::Initial;
		for line in reader.lines() {
			let line = line?;

			match &mut state {
				State::Properties { len } if *len > 0 => {
					let idx: usize = line
						.chars()
						.take_while(|ch| !ch.is_ascii_whitespace())
						.map(|ch| ch.len_utf8())
						.sum();
					let key = &line[0 .. idx];
					let value_str = &line[idx + 1 ..];
					let v = if value_str.starts_with('"') && value_str.ends_with('"') {
						Value::String(value_str.trim_matches('"').replace("''", "\""))
					} else {
						Value::Integer(
							value_str.parse().map_err(Error::InvalidPropertyValue)?
						)
					};
					font_properties.insert(key.to_owned(), v);

					*len -= 1;
					continue;
				},
				_ => {}
			}

			let token = match Token::parse_line(&line)? {
				Some(token) => token,
				None => continue
			};

			match token {
				Token::StartFont { .. } => {
					state.assert_initial(&token)?;
					state = State::Font;
				},

				Token::ContentVersion { ver } => {
					state.assert_font(&token)?;
					font_version = Some(ver);
				},

				Token::Font { ref name } => {
					state.assert_font(&token)?;
					font_name = Some(name.into());
				},

				Token::Size { pt, xres, yres } => {
					state.assert_font(&token)?;
					font_size = Some(Size { pt, xres, yres });
				},

				Token::FontBoundingBox {
					fbbx,
					fbby,
					xoff,
					yoff
				} => {
					state.assert_font(&token)?;
					font_bbox = Some(BoundingBox {
						width: fbbx,
						height: fbby,
						offset_x: xoff,
						offset_y: yoff
					});
				},

				Token::MetricsSet {
					dir: WritingDirection::Horizontal
				} => {},
				Token::MetricsSet { dir } => {
					unimplemented!("METRICSSET {dir:?} is currently not supported");
				},

				Token::SWidth { swx0, swy0 } if matches!(state, State::Font) => {
					font_swidth = Some((swx0, swy0));
				},
				Token::SWidth { swx0, swy0 } => {
					state.assert_glyph(&token)?;
					glyph_swidth = Some((swx0, swy0));
				},

				Token::DWidth { dwx0, dwy0 } if matches!(state, State::Font) => {
					font_dwidth = Some((dwx0, dwy0));
				},
				Token::DWidth { dwx0, dwy0 } => {
					state.assert_glyph(&token)?;
					glyph_dwidth = Some((dwx0, dwy0));
				},

				Token::SWidthVertical { swx1, swy1 } => {
					unimplemented!("SWIDTH1 {swx1} {swy1} is currently not supported");
				},
				Token::DWidthVertical { dwx1, dwy1 } => {
					unimplemented!("DWIDTH1 {dwx1} {dwy1} is currently not supported");
				},
				Token::VVector { xoff, yoff } => {
					unimplemented!("VVECTOR {xoff} {yoff} is currently not supported");
				},

				Token::StartProperties { n } => {
					state.assert_font(&token)?;
					state = State::Properties { len: n };
				},

				Token::EndProperties {} => {
					let len = state.assert_properties(&token)?;
					if len != 0 {
						return Err(Error::UnexpectedEnd("Properties"));
					}
					state = State::Font;
				},

				Token::Chars { nglyphs } => {
					state.assert_font(&token)?;
					state = State::Chars { len: nglyphs };
				},

				Token::StartChar { ref name } => {
					let chars = state.assert_chars(&token)?;
					state = State::Glyph { chars };

					glyph_name = Some(name.to_owned());
					glyph_encoding = None;
					glyph_swidth = font_swidth;
					glyph_dwidth = font_dwidth;
					glyph_bbox = font_bbox;
				},

				Token::Encoding { enc } => {
					state.assert_glyph(&token)?;
					glyph_encoding = Some(enc);
				},

				Token::BoundingBox {
					bbw,
					bbh,
					bbxoff,
					bbyoff
				} => {
					state.assert_glyph(&token)?;
					glyph_bbox = Some(BoundingBox {
						width: bbw,
						height: bbh,
						offset_x: bbxoff,
						offset_y: bbyoff
					});
				},

				Token::EndChar {} => {
					let chars = state.assert_glyph(&token)?;
					state = State::Chars { len: chars - 1 };

					font_glyphs.insert(
						Glyph {
							name: glyph_name.take().unwrap(),
							encoding: glyph_encoding
								.ok_or(Error::MissingGlyphEncoding)?
						}
						.into()
					);
				},

				Token::EndFont {} => {
					let chars = state.assert_chars(&token)?;
					if chars != 0 {
						return Err(Error::UnexpectedEnd("Font"));
					}
					state = State::Final;
				},

				// ignored
				Token::Comment { .. } => {}
			};
		}

		// TODO check that state = final
		Ok(Self {
			version: font_version,
			name: font_name.ok_or(Error::MissingFontName)?,
			bbox: font_bbox.ok_or(Error::MissingFontBoundingBox)?,
			size: font_size.ok_or(Error::MissingFontSize)?,
			properties: font_properties,
			glyphs: font_glyphs
		})
	}
}
