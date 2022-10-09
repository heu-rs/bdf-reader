#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundingBox {
	pub width: u32,
	pub height: u32,
	pub offset_x: i32,
	pub offset_y: i32
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
	pub pt: u32,
	pub xres: u32,
	pub yres: u32
}

pub struct Font {
	pub(crate) version: Option<i32>,
	pub(crate) name: String,
	pub(crate) bbox: BoundingBox,
	pub(crate) size: Size
}

impl Font {
	/// Get the content version of the font.
	pub fn version(&self) -> Option<i32> {
		self.version
	}

	/// Get the name of the font.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Get the default bounding box for glyphs.
	pub fn bounding_box(&self) -> BoundingBox {
		self.bbox
	}

	/// Get the size of the font.
	pub fn size(&self) -> Size {
		self.size
	}
}
