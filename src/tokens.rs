use paste::paste;
use std::str::FromStr;
use thiserror::Error;

macro_rules! tokens {
	(
		$(#[doc$($doc:tt)*])*
		$vis:vis enum $ident:ident {
			$(
				$(#[doc$($variant_doc:tt)*])*
				$(#[test($test_input:literal, $test_expected:expr)])*
				$variant:ident { $tag:literal $(, $arg:ident: $arg_ty:ident)* $(, ..$remaining:ident)? }
			),*
		}
	) => {
		paste! {
			$(#[doc$($doc)*])*
			#[allow(clippy::derive_partial_eq_without_eq)]
			#[derive(Clone, Debug, PartialEq)]
			$vis enum $ident {
				$(
					$(#[doc$($variant_doc)*])*
					$variant { $($arg: $arg_ty,)* $($remaining: String)? }
				),*
			}

			#[derive(Debug, Error)]
			pub enum Error {
				$(
					$(
						#[error(
							"Failed to parse argument {} of tag {}: {0}",
							stringify!($arg),
							$tag
						)]
						[<$variant $arg:camel>](#[source] <$arg_ty as FromStr>::Err),
					)*
				)*

				#[error("Missing the argument {1} of tag {0}")]
				MissingArg(&'static str, &'static str),

				#[error("Extra tokens after tag {0}")]
				ExtraTokens(&'static str),

				#[error("Unknown tag: {0}")]
				UnknownTag(String)
			}

			impl $ident {
				#[allow(unused_assignments, unused_mut)]
				$vis fn parse_line(line: &str) -> Result<Option<Self>, Error> {
					let mut tokens = line.split(|ch: char| ch.is_ascii_whitespace()).peekable();
					if tokens.peek().is_none() {
						return Ok(None);
					}

					$(
						if tokens.peek() == Some(&$tag) {
							tokens.next().unwrap();
							let mut empty = tokens.peek().is_none();
							$(
								let $arg: $arg_ty = tokens
									.next()
									.ok_or(Error::MissingArg($tag, stringify!($arg)))?
									.parse()
									.map_err(|err| Error::[<$variant $arg:camel>](err))?;
								empty = tokens.peek().is_none();
							)*
							$(
								let $remaining = tokens
									.fold(String::new(), |mut rem, token| {
										if !rem.is_empty() {
											rem += " ";
										}
										rem += token;
										rem
									});
								empty = true;
							)?
							if !empty {
								return Err(Error::ExtraTokens($tag));
							}
							return Ok(Some(Self::$variant { $($arg,)* $($remaining)? }));
						}
					)*

					return Err(Error::UnknownTag(tokens.next().unwrap().into()));
				}
			}

			$(
				#[cfg(test)]
				#[test]
				fn [<test_ $ident:lower _parse_ $variant:lower>]() {
					$(
						let expected: Option<$ident> = Some({
							use $ident::*;
							$test_expected
						});
						assert_eq!(expected, $ident::parse_line($test_input).unwrap());
					)*
				}
			)*
		}
	};
}

tokens! {
	pub enum Token {
		/// `STARTFONT` is followed by a version number indicating the exact file format
		/// used (for example, 2.1).
		#[test("STARTFONT 2.1", StartFont { ver: "2.1".into() })]
		StartFont { "STARTFONT", ver: String },

		/// One or more lines beginning with the word `COMMENT`. These lines can be
		/// ignored by any program reading the file.
		#[test("COMMENT hello world", Comment { comment: "hello world".into() })]
		Comment { "COMMENT", ..comment },

		/// (Optional) The value of `CONTENTVERSION` is an integer which can be
		/// assigned by an installer program to keep track of the version of the included
		/// data. The value is intended to be valid only in a single environment, under
		/// the control of a single installer. The value of `CONTENTVERSION` should only
		/// reflect upgrades to the quality of the bitmap images, not to the glyph
		/// complement or encoding.
		#[test("CONTENTVERSION 1", ContentVersion { ver: 1 })]
		ContentVersion { "CONTENTVERSION", ver: i32 },

		/// `FONT` is followed by the font name, which should exactly match the
		/// Post-Script™ language **FontName** in the corresponding outline font program
		#[test("FONT font-name", Font { name: "font-name".into() })]
		Font { "FONT", name: String },

		/// `SIZE` is followed by the point size of the glyphs and the x and y resolutions
		/// of the device for which the font is intended.
		#[test("SIZE 16 75 75", Size { pt: 16, xres: 75, yres: 75 })]
		Size { "SIZE", pt: u32, xres: u32, yres: u32 },

		/// `FONTBOUNDINGBOX` is followed by the width in x and the height in y, and
		/// the x and y displacement of the lower left corner from origin 0 (for
		/// horizontal writing direction); all in integer pixel values.
		#[test(
			"FONTBOUNDINGBOX 16 16 0 -2",
			FontBoundingBox { fbbx: 16, fbby: 16, xoff: 0, yoff: -2 }
		)]
		FontBoundingBox { "FONTBOUNDINGBOX", fbbx: u32, fbby: u32, xoff: i32, yoff: i32 },

		// TODO metricsset

		/// The optional word `STARTPROPERTIES` may be followed by the number of
		/// properties (n) that follow. Within the properties list, there may be n lines
		/// consisting of a word for the property name followed by either an integer or
		/// string surrounded by ASCII double quotation marks (ASCII octal 042).
		/// Internal quotation characters are indicated (or “quoted”) by using two
		/// quotation characters in a row.
		#[test("STARTPROPERTIES 1", StartProperties { n: 1 })]
		StartProperties { "STARTPROPERTIES", n: usize },

		/// The word `ENDPROPERTIES` is used to delimit the end of the optional properties
		/// list in fonts files containing the word `STARTPROPERTIES`.
		#[test("ENDPROPERTIES", EndProperties {})]
		EndProperties { "ENDPROPERTIES" }
	}
}
