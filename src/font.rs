use std::{
	collections::HashMap,
	fmt::{self, Debug, Display, Formatter}
};

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

#[derive(Clone)]
pub enum Value {
	Integer(i32),
	String(String)
}

impl Debug for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Integer(i) => write!(f, "{i}"),
			Self::String(str) => write!(f, "{str:?}")
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Integer(i) => write!(f, "{i}"),
			Self::String(str) => write!(f, "{str}")
		}
	}
}

pub struct Font {
	pub(crate) version: Option<i32>,
	pub(crate) name: String,
	pub(crate) bbox: BoundingBox,
	pub(crate) size: Size,
	pub(crate) properties: HashMap<String, Value>
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

	/// Get a property of the font.
	pub fn property(&self, key: &str) -> Option<&Value> {
		self.properties.get(key)
	}
}
