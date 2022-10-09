use crate::{tokens::Token, BoundingBox, Error, Font, Size};
use std::io::BufRead;

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

		let mut state = State::Initial;
		for line in reader.lines() {
			let token = match Token::parse_line(&line?)? {
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

				// ignored
				Token::Comment { .. } => {},

				_ => unimplemented!()
			};
		}

		Ok(Self {
			version: font_version,
			name: font_name.ok_or(Error::MissingFontName)?,
			bbox: font_bbox.ok_or(Error::MissingFontBoundingBox)?,
			size: font_size.ok_or(Error::MissingFontSize)?
		})
	}
}
