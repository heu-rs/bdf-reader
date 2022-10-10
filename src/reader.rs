use crate::{tokens::Token, BoundingBox, Error, Font, Size, Value};
use std::{collections::HashMap, io::BufRead};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
	/// Initial state.
	Initial,

	/// Between start and end font.
	Font,

	/// Inside the font's properties, with `len` properties remaining.
	Properties { len: usize }
}

fn assert_state(token: &Token, actual: State, expected: State) -> Result<(), Error> {
	if actual != expected {
		return Err(Error::InvalidContext(token.clone(), actual, expected));
	}
	Ok(())
}

impl Font {
	pub fn read<R: BufRead>(reader: R) -> Result<Self, Error> {
		let mut font_version = None;
		let mut font_name = None;
		let mut font_bbox = None;
		let mut font_size = None;
		let mut font_properties = HashMap::new();

		let mut state = State::Initial;
		for line in reader.lines() {
			let line = line?;

			match state {
				State::Properties { len } if len > 0 => {
					let idx: usize = line
						.chars()
						.take_while(|ch| !ch.is_ascii_whitespace())
						.map(|ch| ch.len_utf8())
						.sum();
					let key = &line[0 .. idx];
					let value_str = &line[idx + 1 ..];
					let value = if value_str.starts_with('"') && value_str.ends_with('"')
					{
						Value::String(value_str.trim_matches('"').replace("''", "\""))
					} else {
						Value::Integer(
							value_str.parse().map_err(Error::InvalidPropertyValue)?
						)
					};
					font_properties.insert(key.to_owned(), value);
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
					assert_state(&token, state, State::Initial)?;
					state = State::Font;
				},

				Token::ContentVersion { ver } => {
					assert_state(&token, state, State::Font)?;
					font_version = Some(ver);
				},

				Token::Font { ref name } => {
					assert_state(&token, state, State::Font)?;
					font_name = Some(name.into());
				},

				Token::Size { pt, xres, yres } => {
					assert_state(&token, state, State::Font)?;
					font_size = Some(Size { pt, xres, yres });
				},

				Token::FontBoundingBox {
					fbbx,
					fbby,
					xoff,
					yoff
				} => {
					assert_state(&token, state, State::Font)?;
					font_bbox = Some(BoundingBox {
						width: fbbx,
						height: fbby,
						offset_x: xoff,
						offset_y: yoff
					});
				},

				Token::StartProperties { n } => {
					assert_state(&token, state, State::Font)?;
					state = State::Properties { len: n };
				},

				Token::EndProperties {} => {
					assert_state(&token, state, State::Properties { len: 0 })?;
					state = State::Font;
				},

				// ignored
				Token::Comment { .. } => {}
			};
		}

		Ok(Self {
			version: font_version,
			name: font_name.ok_or(Error::MissingFontName)?,
			bbox: font_bbox.ok_or(Error::MissingFontBoundingBox)?,
			size: font_size.ok_or(Error::MissingFontSize)?,
			properties: font_properties
		})
	}
}
