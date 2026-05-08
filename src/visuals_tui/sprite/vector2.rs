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

        impl ops::AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$i += rhs.$i;)+
            }
        }
    };
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2u {
    pub x: u32,
    pub y: u32,
}

impl ops::SubAssign for Vector2u {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x.saturating_sub(rhs.x);
        self.y = self.y.saturating_sub(rhs.y)
    }
}

impl Vector2u {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl_vector_op!(Vector2i, x, y);
impl_vector_op!(Vector2u, x, y);
