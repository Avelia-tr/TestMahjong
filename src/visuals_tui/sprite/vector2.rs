use std::ops;

macro_rules! impl_vector_op {
    ($t:ty, $($i:ident),+) => {
        impl ops::Add for $t{
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($i : self.$i + rhs.$i,)+
                }
            }
        }

        impl ops::Sub for $t{
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($i : self.$i - rhs.$i,)+
                }
            }
        }

        impl ops::Mul for $t{
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($i : self.$i * rhs.$i,)+
                }
            }
        }
    };
}

#[derive(Clone, Copy)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Vector2u {
    pub x: u32,
    pub y: u32,
}

impl_vector_op!(Vector2i, x, y);
impl_vector_op!(Vector2u, x, y);
