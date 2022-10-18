use paste::paste;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("No such variant: {0}")]
pub struct NoSuchVariant(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WritingDirection {
	Horizontal = 0,
	Vertical = 1,
	Both = 2
}

impl FromStr for WritingDirection {
	type Err = NoSuchVariant;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"0" => Ok(Self::Horizontal),
			"1" => Ok(Self::Vertical),
			"2" => Ok(Self::Both),
			_ => Err(NoSuchVariant(s.into()))
		}
	}
}

macro_rules! tokens {
	(
		$(#[doc$($doc:tt)*])*
		$vis:vis enum $ident:ident {
			$(
				$(#[doc$($variant_doc:tt)*])*
				$(#[test($test_input:literal, $test_expected:expr)])*
				$variant:ident {
					$tag:literal $(,
						$($arg:ident: $arg_ty:ident),* $(,)?
						$(..$remaining:ident)?
					)?
				}
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
					$variant $({ $($arg: $arg_ty,)* $($remaining: String)? })?
				),*
			}

			#[derive(Debug, Error)]
			pub enum Error {
				$(
					$(
						$(
							#[error(
								"Failed to parse argument {} of tag {}: {0}",
								stringify!($arg),
								$tag
							)]
							[<$variant $arg:camel>](#[source] <$arg_ty as FromStr>::Err),
						)*
					)?
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
				$vis fn parse_line(line: &str) -> Result<Self, Error> {
					let mut tokens = line
						.trim_end()
						.split(|ch: char| ch.is_ascii_whitespace())
						.peekable();

					$(
						if tokens.peek() == Some(&$tag) {
							tokens.next().unwrap();
							let mut empty = tokens.peek().is_none();
							$(
								$(
									let $arg: $arg_ty = tokens
										.next()
										.ok_or(Error::MissingArg($tag, stringify!($arg)))?
										.parse()
										.map_err(Error::[<$variant $arg:camel>])?;
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
							)?
							if !empty {
								return Err(Error::ExtraTokens($tag));
							}
							return Ok(Self::$variant $({ $($arg,)* $($remaining)? })?);
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
						let expected: $ident = {
							use $ident::*;
							$test_expected
						};
						assert_eq!(expected, $ident::parse_line($test_input).unwrap());
					)*
				}
			)*
		}
	};
}

// https://adobe-type-tools.github.io/font-tech-notes/pdfs/5005.BDF_Spec.pdf
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

		/// (Optional) The integer value of `METRICSSET  may be 0, 1, or 2, which
		/// scorrepond to writing direction 0 only, 1 only, or both (respectively). If not
		/// present, `METRICSSET` 0 is implied. If `METRICSSET` is 1, `DWIDTH` and
		/// `SWIDTH` keywords are optional.
		#[test("METRICSSET 0", MetricsSet { dir: WritingDirection::Horizontal })]
		#[test("METRICSSET 1", MetricsSet { dir: WritingDirection::Vertical })]
		#[test("METRICSSET 2", MetricsSet { dir: WritingDirection::Both })]
		MetricsSet { "METRICSSET", dir: WritingDirection },

		/// `SWIDTH` is followed by swx0 and swy0, the scalable width of the glyph in x
		/// and y for writing mode 0. The scalable widths are of type Number and are in
		/// units of 1/1000th of the size of the glyph and correspond to the widths found
		/// in AFM files (for outline fonts). If the size of the glyph is p points, the
		/// width information must be scaled by p/1000 to get the width of the glyph in
		/// printer’s points. This width information should be regarded as a vector
		/// indicating the position of the next glyph’s origin relative to the origin of
		/// this glyph. `SWIDTH` is mandatory for all writing mode 0 fonts.
		///
		/// To convert the scalable width to the width in device pixels, multiply `SWIDTH`
		/// times p/1000 times r/72, where r is the device resolution in pixels per inch.
		/// The result is a real number giving the ideal width in device pixels. The
		/// actual device width must be an integral number of device pixels and is given
		/// by the `DWIDTH` entry.
		#[test("SWIDTH 1000 0", SWidth { swx0: 1000.0, swy0: 0.0 })]
		SWidth { "SWIDTH", swx0: f64, swy0: f64 },

		/// `DWIDTH` specifies the widths in x and y, dwx0 and dwy0, in device pixels.
		/// Like `SWIDTH`, this width information is a vector indicating the position of
		/// the next glyph’s origin relative to the origin of this glyph. `DWIDTH` is
		/// mandatory for all writing mode 0 fonts.
		#[test("DWIDTH 16 0", DWidth { dwx0: 16.0, dwy0: 0.0 })]
		DWidth { "DWIDTH", dwx0: f64, dwy0: f64 },

		/// `SWIDTH1` is followed by the values for swx1 and swy1, the scalable width of
		/// the glyph in x and y, for writing mode 1 (vertical direction). The values are
		/// of type Number, and represent the widths in glyph space coordinates.
		#[test("SWIDTH1 1000 0", SWidthVertical { swx1: 1000.0, swy1: 0.0 })]
		SWidthVertical { "SWIDTH1", swx1: f64, swy1: f64 },

		/// `DWIDTH1` specifies the integer pixel width of the glyph in x and y. Like
		/// `SWIDTH1`, this width information is a vector indicating the position of the
		/// next glyph’s origin relative to the origin of this glyph. `DWIDTH1` is
		/// mandatory for all writing mode 1 fonts.
		#[test("DWIDTH1 16 0", DWidthVertical { dwx1: 16.0, dwy1: 0.0 })]
		DWidthVertical { "DWIDTH1", dwx1: f64, dwy1: f64 },

		/// `VVECTOR` (optional) specifies the components of a vector from origin 0 (the
		/// origin for writing direction 0) to origin 1 (the origin for writing direction
		/// 1). If the value of `METRICSSET` is 1 or 2, `VVECTOR` must be specified either
		/// at the global level, or for each individual glyph. If specified at the global
		/// level, the `VVECTOR` is the same for all glyphs, though the inclusion of this
		/// keyword in an individual glyph has the effect of overriding the bal value for
		/// that specific glyph.
		#[test("VVECTOR 1 1", VVector { xoff: 1.0, yoff: 1.0 })]
		VVector { "VVECTOR", xoff: f64, yoff: f64 },

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
		EndProperties { "ENDPROPERTIES" },

		/// `CHARS` is followed by *nglyphs*, the number of glyphs that follow. To make
		/// sure that the correct number of glyphs were actually read and processed, error
		/// checking is recommended at the end of the file
		#[test("CHARS 1", Chars { nglyphs: 1 })]
		Chars { "CHARS", nglyphs: usize },

		/// The word `STARTCHAR` followed by a string containing the name for the
		/// glyph. In base fonts, this should correspond to the name in the PostScript
		/// language outline font’s encoding vector. In a Composite font (Type 0), the
		/// value may be a numeric offset or glyph ID.
		#[test("STARTCHAR U+0041", StartChar { name: "U+0041".into() })]
		StartChar { "STARTCHAR", name: String },

		/// `ENCODING` is followed by a positive integer representing the Adobe Standard
		/// Encoding value. If the character is not in the Adobe Standard Encoding,
		/// `ENCODING` is followed by –1 and optionally by another integer specifying
		/// the glyph index for the non-standard encoding.
		// TODO represent the optional encoding value
		#[test("ENCODING 65", Encoding { enc: 65 })]
		Encoding { "ENCODING", enc: u32 },

		/// `BBX` is followed by BBw, the width of the black pixels in x, and BBh, the
		/// height in y. These are followed by the x and y displacement, BBxoff0 and
		/// BByoff0, of the lower left corner of the bitmap from origin 0. All values are
		/// are an integer number of pixels.
		///
		/// If the font specifies metrics for writing direction 1, `VVECTOR` specifies the
		/// offset from origin 0 to origin 1. For example, for writing direction 1, the
		/// offset from origin 1 to the lower left corner of the bitmap would be:
		///
		/// BBxoff1x,y = BBxoff0x,y – `VVECTOR`
		#[test("BBX 16 16 0 -2", BoundingBox { bbw: 16, bbh: 16, bbxoff: 0, bbyoff: -2 })]
		BoundingBox { "BBX", bbw: u32, bbh: u32, bbxoff: i32, bbyoff: i32 },

		/// `BITMAP` introduces the hexadecimal data for the character bitmap. From the
		/// `BBX` value for h, find h lines of hex-encoded bitmap, padded on the right
		/// with zero’s to the nearest byte (that is, multiple of 8). Hex data can be
		/// turned into binary by taking two bytes at a time, each of which represents 4
		/// bits of the 8-bit value. For example, the byte 01101101 is two hex digits: 6
		/// (0110 in hex) and D (1101 in hex).
		#[test("BITMAP", Bitmap {})]
		Bitmap { "BITMAP" },

		/// `ENDCHAR` delimits the end of the glyph description
		#[test("ENDCHAR", EndChar {})]
		EndChar { "ENDCHAR" },

		/// The entire file is terminated with the word `ENDFONT`. If this is encountered
		/// before all of the glyphs have been read, it is an error cond
		#[test("ENDFONT", EndFont {})]
		EndFont { "ENDFONT" }
	}
}
