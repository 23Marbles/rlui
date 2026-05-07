use num_traits::{PrimInt, Signed};

pub mod cursor;
pub mod keys;

#[doc(hidden)]
pub trait IntPos: PrimInt {
    type Signed: PrimInt + Signed;

    fn abs(signed: Self::Signed) -> Self;
}

macro_rules! impl_int_pos {
    ($base:ty, $sign:ty) => {
        impl IntPos for $base {
            type Signed = $sign;

            fn abs(signed: Self::Signed) -> Self {
                signed.abs() as $base
            }
        }
    };
}

impl_int_pos!(u8, i8);
impl_int_pos!(u16, i16);
impl_int_pos!(u32, i32);
impl_int_pos!(u64, i64);
impl_int_pos!(usize, isize);

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Pos<C: IntPos> {
    pub col: C,
    pub row: C,
}
