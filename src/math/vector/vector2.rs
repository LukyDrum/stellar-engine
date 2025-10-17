use num_traits::Float;
use paste::paste;
use std::ops::RemAssign;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use crate::math::vector::VectorItem;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2<T>
where
    T: VectorItem,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: VectorItem,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.length_squared().sqrt()
    }
}

/// Used to generate basic **vector** mathematical operations for vectors.
macro_rules! impl_operators_vector2 {
    ($trait:ty, $op:tt, $aop:tt) => {
        paste! {
            impl<T> $trait for Vector2<T> where T: VectorItem {
                type Output = Self;

                fn [<$trait:lower>](self, rhs: Self) -> Self::Output {
                    Self {
                        x: self.x $op rhs.x,
                        y: self.y $op rhs.y,
                    }
                }
            }

            impl<T> [<$trait Assign>] for Vector2<T> where T: VectorItem {
                fn [<$trait:lower _assign>](&mut self, rhs: Self) {
                    self.x $aop rhs.x;
                    self.y $aop rhs.y;
                }
            }
        }
    };
}

/// Used to generate basic **scalar** mathematical operations for vectors.
macro_rules! impl_scalar_operators_vector2 {
    ($trait:ty, $op:tt, $aop:tt) => {
        paste! {
            impl<T> $trait<T> for Vector2<T> where T: VectorItem {
                type Output = Self;

                fn [<$trait:lower>](self, rhs: T) -> Self::Output {
                    Self {
                        x: self.x $op rhs,
                        y: self.y $op rhs,
                    }
                }
            }

            impl<T> [<$trait Assign>]<T> for Vector2<T> where T: VectorItem {
                fn [<$trait:lower _assign>](&mut self, rhs: T) {
                    self.x $aop rhs;
                    self.y $aop rhs;
                }
            }
        }
    };
}

impl_operators_vector2!(Add, +, +=);
impl_operators_vector2!(Sub, -, -=);
impl_scalar_operators_vector2!(Mul, *, *=);
impl_scalar_operators_vector2!(Div, /, /=);
impl_scalar_operators_vector2!(Rem, %, %=);

impl<T> Neg for Vector2<T>
where
    T: VectorItem + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
