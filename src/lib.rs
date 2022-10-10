#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

use std::{io, str::FromStr};
use thiserror::Error;

mod font;
mod reader;
mod tokens;

pub use font::{BoundingBox, Font, Size, Value};
use reader::State;
use tokens::Token;

#[derive(Debug, Error)]
pub enum Error {
	#[error("I/O Error: {0}")]
	IOError(#[from] io::Error),

	#[error("Syntax Error: {0}")]
	SyntaxError(#[from] tokens::Error),

	#[error("The token {0:?} may only appear in {2:?}, but is in {1:?}")]
	InvalidContext(Token, State, State),

	#[error("Missing font name")]
	MissingFontName,

	#[error("Missing font size")]
	MissingFontSize,

	#[error("Missing font bounding box")]
	MissingFontBoundingBox,

	#[error("Invalid Property Value: {0}. Note that strings need to be quoted.")]
	InvalidPropertyValue(#[source] <i32 as FromStr>::Err)
}
