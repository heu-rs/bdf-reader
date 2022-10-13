use bit_vec::BitVec;

#[derive(Clone, Debug)]
pub struct Bitmap {
	pub(crate) data: Vec<BitVec>
}
