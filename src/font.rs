use crate::Bitmap;
use bit_vec::BitVec;
use std::{
	borrow::Borrow,
	cmp::Ordering,
	collections::{BTreeSet, HashMap},
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

#[derive(Clone, Debug)]
pub struct Glyph {
	pub(crate) name: String,
	pub(crate) encoding: u32,
	pub(crate) swidth: Option<(f64, f64)>,
	pub(crate) dwidth: Option<(f64, f64)>,
	pub(crate) bbox: BoundingBox,
	pub(crate) bitmap: Vec<BitVec>
}

impl Glyph {
	/// Get the name of this glyph as specified in the font.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Get the encoding value of this glyph.
	pub fn encoding(&self) -> u32 {
		self.encoding
	}

	/// Get the space width of this glyph.
	pub fn swidth(&self) -> Option<(f64, f64)> {
		self.swidth
	}

	/// Get the device width of this glyph.
	pub fn dwidth(&self) -> Option<(f64, f64)> {
		self.dwidth
	}

	/// Get the bounding box of this glyph.
	pub fn bounding_box(&self) -> BoundingBox {
		self.bbox
	}

	/// Get the bitmap of this glyph.
	pub fn bitmap(&self) -> Bitmap<'_> {
		Bitmap {
			data: &self.bitmap,
			bbox: self.bbox
		}
	}
}

/// A glyph wrapper that can be compared by its encoding.
pub(crate) struct GlyphWrapper(Glyph);

impl From<Glyph> for GlyphWrapper {
	fn from(glyph: Glyph) -> Self {
		Self(glyph)
	}
}

impl Borrow<u32> for GlyphWrapper {
	fn borrow(&self) -> &u32 {
		&self.0.encoding
	}
}

impl PartialEq for GlyphWrapper {
	fn eq(&self, other: &Self) -> bool {
		self.0.encoding == other.0.encoding
	}
}

impl Eq for GlyphWrapper {}

impl PartialEq<u32> for GlyphWrapper {
	fn eq(&self, other: &u32) -> bool {
		self.0.encoding == *other
	}
}

impl PartialOrd for GlyphWrapper {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GlyphWrapper {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.encoding.cmp(&other.0.encoding)
	}
}

/// A trait to help use u32 and char as glyph index.
pub trait GlyphIdx {
	fn encoding(self) -> u32;
}

impl GlyphIdx for u32 {
	fn encoding(self) -> u32 {
		self
	}
}

impl GlyphIdx for char {
	fn encoding(self) -> u32 {
		self as _
	}
}

pub struct Font {
	pub(crate) version: Option<i32>,
	pub(crate) name: String,
	pub(crate) bbox: BoundingBox,
	pub(crate) size: Size,
	pub(crate) properties: HashMap<String, Value>,

	pub(crate) glyphs: BTreeSet<GlyphWrapper>
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

	/// Get an iterator over all glyphs of the font.
	pub fn glyphs(&self) -> impl IntoIterator<Item = &Glyph> {
		self.glyphs.iter().map(|gw| &gw.0)
	}

	/// Get the glyph for this character, if contained in the font.
	pub fn glyph<I: GlyphIdx>(&self, ch: I) -> Option<&Glyph> {
		self.glyphs.get(&ch.encoding()).map(|gw| &gw.0)
	}
}
