#[macro_export]
macro_rules! matrix {
    // Square
    ($type:tt; $n:expr) => {{
        Matrix::<$n, $n, $type>::new(<$type as IsNumber>::ZERO)
    }};
    // NxM
    ($type:tt; $n:expr; $m:expr) => {{
        Matrix::<$n, $m, $type>::new(<$type as IsNumber>::ZERO)
    }};
    // specific array
    ([$( [ $($elem:expr),* ] ),* ]) => {{
        Matrix {
            elements: [
                $( [ $($elem),* ] ),*
            ]
        }
    }};
}

// check if is a number, check at compile time
pub trait IsNumber {
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_trait {
    ($trait: tt, [$($types:tt )+]) => {
        $(
            impl $trait for $types {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )+
    };
}

impl_trait!(IsNumber, [usize isize u8 i8 u16 i16 i32 u32 i64 u64]);
