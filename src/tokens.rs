#![allow(dead_code)]

#[cfg(not(test))]
macro_rules! tokens {
	($($tt:tt)*) => {
		tokens_impl!($($tt)*);
	}
}

#[cfg(test)]
macro_rules! tokens {
	($($tt:tt)*) => {
		tokens_impl!($($tt)*);
		tokens_tests!($($tt)*);
	}
}

macro_rules! tokens_impl {
	// keep this in sync with tokens_tests!
	(
		$(#[doc$($doc:tt)*])*
		$vis:vis enum $ident:ident {
			$(
				$(#[doc$($variant_doc:tt)*])*
				$(#[test($test_input:literal, $test_expected:expr)])*
				$variant:ident { $tag:literal $(, $arg:ident: $arg_ty:ty)* }
			),*
		}
	) => {
		$(#[doc$($doc)*])*
		#[derive(Debug, PartialEq)]
		$vis enum $ident {
			$(
				$(#[doc$($variant_doc)*])*
				$variant { $($arg: $arg_ty),* }
			),*
		}

		#[derive(Debug)]
		$vis struct Error;

		impl $ident {
			$vis fn parse_line(line: &str) -> Result<Self, Error> {
				let mut tokens = line.split(|ch: char| ch.is_ascii_whitespace()).peekable();
				$(
					if tokens.peek() == Some(&$tag) {
						_ = tokens.next();
						$(
							let $arg: $arg_ty = tokens.next().ok_or(Error)?.parse().map_err(|_| Error)?;
						)*
						return Ok(Self::$variant { $($arg),* });
					}
				)*

				return Err(Error);
			}
		}
	};
}

#[cfg(test)]
macro_rules! tokens_tests {
	(
		$(#[doc$($doc:tt)*])*
		$vis:vis enum $ident:ident {
			$(
				$(#[doc$($variant_doc:tt)*])*
				$(#[test($test_input:literal, $test_expected:expr)])*
				$variant:ident { $tag:literal $(, $arg:ident: $arg_ty:ty)* }
			),*
		}
	) => {
		paste::paste! {
			$(
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

tokens! {
	pub(crate) enum Token {
		#[test("STARTFONT 2.1", StartFont { ver: "2.1".into() })]
		StartFont { "STARTFONT", ver: String }
	}
}
