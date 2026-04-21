use std::iter::Sum;

use num_traits::{Float, Num, Signed, ToPrimitive};

#[macro_export]
macro_rules! matrix {
    // Square
    ($type:tt; $n:expr) => {{
        $crate::Matrix::<$n, $n, $type>::new(<$type as IsNumber>::ZERO)
    }};
    // NxM
    ($type:tt; $n:expr; $m:expr) => {{
        $crate::Matrix::<$n, $m, $type>::new(<$type as IsNumber>::ZERO)
    }};
    // specific array
    ($( [ $($elem:expr),* ] ),*) => {{
      $crate::Matrix {
            elements: [
                $( [ $($elem),* ] ),*
            ]
        }
    }};
}

pub trait IsNumber: Num + Copy + PartialOrd + Sum<Self> + ToPrimitive {
    const ZERO: Self;
    const ONE: Self;
}

pub trait IsSigned: IsNumber + Signed {}

pub trait IsReal: IsSigned + Float {}

macro_rules! impl_is_number {
    ([$($types:ty)+]) => {
        $(
            impl IsNumber for $types {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )+
    };
}

macro_rules! impl_is_signed {
    ([$($types:ty)+]) => {
        $(
            impl IsNumber for $types {
                const ZERO: Self = 0 as Self;
                const ONE: Self = 1 as Self;
            }
            impl IsSigned for $types {}
        )+
    };
}

macro_rules! impl_is_real {
    ([$($types:ty)+]) => {
        $(
            impl IsReal for $types {}
        )+
    };
}

impl_is_number!([usize u8 u16 u32 u64]);
impl_is_signed!([isize i8 i16 i32 i64 f32 f64]);
impl_is_real!([f32 f64]);
