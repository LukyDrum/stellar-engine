use num_traits::{Num, NumAssign, NumAssignOps, NumAssignRef};
use std::fmt::Debug;

/// This trait defines the requirements for types than can be used in Vector structs.
/// It is implemented for all base numerical types.
pub trait VectorItem:
    Num + NumAssign + NumAssignOps + NumAssignRef + Clone + Copy + Debug + Default
{
}

impl VectorItem for i8 {}
impl VectorItem for i16 {}
impl VectorItem for i32 {}
impl VectorItem for i64 {}
impl VectorItem for u8 {}
impl VectorItem for u16 {}
impl VectorItem for u32 {}
impl VectorItem for u64 {}
impl VectorItem for f32 {}
impl VectorItem for f64 {}
