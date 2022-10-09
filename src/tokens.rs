#![allow(dead_code)]

macro_rules! tokens {
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
			$vis fn parse_line(_line: &str) -> Result<Self, Error> {
				unimplemented!()
			}
		}

		// TODO potentially only codegen this when tests are enabled and use paste?
		#[cfg(test)]
		mod tests {
			use super::$ident;

			$(
				#[test]
				#[allow(non_snake_case)]
				fn $variant() {
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
