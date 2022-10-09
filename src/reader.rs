// TODO remove this
#![allow(dead_code)]

use crate::{tokens::Token, Error};
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

fn read<R: BufRead>(reader: R) -> Result<(), Error> {
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

			Token::Font { .. } => {
				assert_state(&token, state, State::Font)?;
			},

			Token::Size { .. } => {
				assert_state(&token, state, State::Font)?;
			},

			Token::FontBoundingBox { .. } => {
				assert_state(&token, state, State::Font)?;
			},

			// ignored
			Token::Comment { .. } => {},
			Token::ContentVersion { .. } => {},

			_ => unimplemented!()
		};
	}

	Ok(())
}
