#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

//! # BDF Reader
//! 
//! A reader for the BDF ([Glyph Bitmap Distribution Format][wikipedia]) font format.
//! 
//! ## Example
//! 
//! ```rust,edition2021
//! use bdf_reader::Font;
//! use std::{fs::File, io::BufReader};
//! 
//! let reader = BufReader::new(File::open("path/to/font.bdf")?);
//! let font = Font::read(reader)?;
//! ```
//! 
//!  [wikipedia]: https://en.wikipedia.org/wiki/Glyph_Bitmap_Distribution_Format

use std::{io, str::FromStr};
use thiserror::Error;

mod bitmap;
mod font;
mod reader;
mod tokens;

pub use bitmap::Bitmap;
pub use font::{BoundingBox, Font, Glyph, Size, Value};
use reader::State;
use tokens::Token;

#[derive(Debug, Error)]
pub enum Error {
	#[error("I/O Error: {0}")]
	IOError(#[from] io::Error),

	#[error("Syntax Error: {0}")]
	SyntaxError(#[from] tokens::Error),

	#[error("The token {0:?} may only appear in {2:?}, but is in {1:?}")]
	InvalidContext(Token, State, &'static str),

	#[error("Unexpected end of context {0}")]
	UnexpectedEnd(&'static str),

	#[error("Missing font name")]
	MissingFontName,

	#[error("Missing font size")]
	MissingFontSize,

	#[error("Missing font bounding box")]
	MissingFontBoundingBox,

	#[error("Missing glyph encoding")]
	MissingGlyphEncoding,

	#[error("Missing glyph bounding box")]
	MissingGlyphBoundingBox,

	#[error("Invalid Property Value: {0}. Note that strings need to be quoted.")]
	InvalidPropertyValue(#[source] <i32 as FromStr>::Err),

	#[error("Invalid bitmap value: {0}")]
	InvalidBitmapValue(String)
}
