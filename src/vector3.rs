use crate::Number;
use az::Cast;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3 {
    pub x: Number,
    pub y: Number,
    pub z: Number,
}

impl Vector3 {
    pub const ZERO: Vector3 = Self::new(Number::ZERO, Number::ZERO, Number::ZERO);
    pub const ONE: Vector3 = Self::new(Number::ONE, Number::ONE, Number::ONE);
    pub const X: Vector3 = Self::new(Number::ONE, Number::ZERO, Number::ZERO);
    pub const Y: Vector3 = Self::new(Number::ZERO, Number::ONE, Number::ZERO);
    pub const Z: Vector3 = Self::new(Number::ZERO, Number::ZERO, Number::ONE);

    #[inline]
    pub const fn new(x: Number, y: Number, z: Number) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn dot(self, other: Vector3) -> Number {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn sqr_length(self) -> Number {
        self.dot(self)
    }

    #[inline]
    pub fn length(self) -> Number {
        self.sqr_length().sqrt()
    }
}

impl From<Vector3> for cgmath::Vector3<f32> {
    fn from(value: Vector3) -> Self {
        Self::new(value.x.cast(), value.y.cast(), value.z.cast())
    }
}

macro_rules! impl_op {
    ($trait:ident $method:ident $op:tt) => {
        impl $trait<Vector3> for Vector3 {
            type Output = Vector3;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }

        impl $trait<Number> for Vector3 {
            type Output = Vector3;

            #[inline]
            fn $method(self, rhs: Number) -> Self::Output {
                Self {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }

        impl $trait<Vector3> for Number {
            type Output = Vector3;

            #[inline]
            fn $method(self, rhs: Vector3) -> Self::Output {
                Vector3 {
                    x: self $op rhs.x,
                    y: self $op rhs.y,
                    z: self $op rhs.z,
                }
            }
        }
    };
}

impl_op!(Add add +);
impl_op!(Sub sub -);
impl_op!(Mul mul *);
impl_op!(Div div /);

macro_rules! impl_op_assign {
    ($trait:ident $method:ident $op:tt) => {
        impl $trait<Vector3> for Vector3 {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }

        impl $trait<Number> for Vector3 {
            #[inline]
            fn $method(&mut self, rhs: Number) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }
    };
}

impl_op_assign!(AddAssign add_assign +=);
impl_op_assign!(SubAssign sub_assign -=);
impl_op_assign!(MulAssign mul_assign *=);
impl_op_assign!(DivAssign div_assign /=);

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
