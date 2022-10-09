#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

use std::io;
use thiserror::Error;

mod reader;
mod tokens;

use reader::State;
use tokens::Token;

#[derive(Debug, Error)]
pub enum Error {
	#[error("I/O Error: {0}")]
	IOError(#[from] io::Error),

	#[error("Syntax Error: {0}")]
	SyntaxError(#[from] tokens::Error),

	#[error("The token {0:?} may only appear in {2:?}, but is in {1:?}")]
	InvalidContext(Token, State, State)
}
