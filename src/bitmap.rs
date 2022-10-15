use crate::BoundingBox;
use bit_vec::BitVec;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Index ({0}, {1}) Out Of Bounds")]
#[non_exhaustive]
pub struct OutOfBounds(usize, usize);

#[derive(Clone, Copy, Debug)]
pub struct Bitmap<'a> {
	pub(crate) data: &'a Vec<BitVec>,
	pub(crate) bbox: BoundingBox
}

impl Bitmap<'_> {
	pub fn width(self) -> usize {
		self.bbox.width as usize
	}

	pub fn height(self) -> usize {
		self.bbox.height as usize
	}

	pub fn baseline(self) -> usize {
		(self.bbox.height as i32 - 1 + self.bbox.offset_y) as usize
	}

	pub fn get(self, x: usize, y: usize) -> Result<bool, OutOfBounds> {
		let row = self.data.get(y).ok_or(OutOfBounds(x, y))?;
		row.get(x).ok_or(OutOfBounds(x, y))
	}

	pub fn ascii_art(self) -> String {
		let mut buf = String::new();
		for y in 0 .. self.height() {
			for x in 0 .. self.width() {
				if self.get(x, y).unwrap() {
					buf += "##";
				} else {
					buf += "..";
				}
			}
			buf += "\n";

			if y == self.baseline() {
				for _ in 0 .. self.width() {
					buf += "--";
				}
				buf += "\n";
			}
		}
		buf
	}
}
