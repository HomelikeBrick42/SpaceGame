use crate::{sin_cos, vector3::Vector3, Number};
use az::Cast;
use encase::ShaderType;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub e012: Number,
    pub e013: Number,
    pub e023: Number,
    pub e123: Number,
}

impl Point {
    pub const IDENTITY: Self = Self {
        e012: Number::ZERO,
        e013: Number::ZERO,
        e023: Number::ZERO,
        e123: Number::ZERO,
    };

    pub fn transform(self, motor: Transform) -> Self {
        let a = motor.s;
        let b = motor.e12;
        let c = motor.e13;
        let d = motor.e23;
        let e = motor.e01;
        let f = motor.e02;
        let g = motor.e03;
        let h = motor.e0123;
        let i = self.e012;
        let j = self.e013;
        let k = self.e023;
        let l = self.e123;

        /*
        Apply motor to point

        (a + b*e2*e1 + c*e3*e1 + d*e3*e2 + e*e1*e0 + f*e2*e0 + g*e3*e0 + h*e3*e2*e1*e0)
        *(i*e0*e1*e2 + j*e0*e1*e3 + k*e0*e2*e3 + l*e1*e2*e3)
        *(a + b*e1*e2 + c*e1*e3 + d*e2*e3 + e*e0*e1 + f*e0*e2 + g*e0*e3 + h*e0*e1*e2*e3)

        (
              -2*a*d*j + -2*a*g*l +   a*a*i + 2*a*c*k
            + -1*d*d*i + -2*d*f*l + 2*b*d*k + -2*b*h*l
            + -2*c*e*l +    b*b*i + 2*b*c*j + -1*c*c*i
        )*e0*e1*e2
        +
        (
              -2*a*b*k + -1*b*b*j + 2*b*c*i +  2*b*e*l
            +    a*a*j +  2*a*d*i + 2*a*f*l + -2*c*h*l
            + -2*d*g*l + -1*d*d*j + 2*c*d*k +    c*c*j
        )*e0*e1*e3
        +
        (
              -2*a*c*i + -2*a*e*l +   a*a*k +  2*a*b*j
            + -1*c*c*k +  2*c*d*j + 2*c*g*l + -2*d*h*l
            +  2*b*f*l + -1*b*b*k + 2*b*d*i +    d*d*k
        )*e0*e2*e3
        +
        (
            a*a*l + b*b*l + c*c*l + d*d*l
        )*e1*e2*e3

        */

        Self {
            e012: Number::from_num(-2) * a * d * j
                + Number::from_num(-2) * a * g * l
                + a * a * i
                + Number::from_num(2) * a * c * k
                - d * d * i
                + Number::from_num(-2) * d * f * l
                + Number::from_num(2) * b * d * k
                + Number::from_num(-2) * b * h * l
                + Number::from_num(-2) * c * e * l
                + b * b * i
                + Number::from_num(2) * b * c * j
                - c * c * i,
            e013: Number::from_num(-2) * a * b * k - b * b * j
                + Number::from_num(2) * b * c * i
                + Number::from_num(2) * b * e * l
                + a * a * j
                + Number::from_num(2) * a * d * i
                + Number::from_num(2) * a * f * l
                + Number::from_num(-2) * c * h * l
                + Number::from_num(-2) * d * g * l
                - d * d * j
                + Number::from_num(2) * c * d * k
                + c * c * j,
            e023: Number::from_num(-2) * a * c * i
                + Number::from_num(-2) * a * e * l
                + a * a * k
                + Number::from_num(2) * a * b * j
                - c * c * k
                + Number::from_num(2) * c * d * j
                + Number::from_num(2) * c * g * l
                + Number::from_num(-2) * d * h * l
                + Number::from_num(2) * b * f * l
                - b * b * k
                + Number::from_num(2) * b * d * i
                + d * d * k,
            e123: a * a * l + b * b * l + c * c * l + d * d * l,
        }
    }
}

impl From<Vector3> for Point {
    fn from(value: Vector3) -> Self {
        Self {
            e012: value.z,
            e013: -value.y,
            e023: value.x,
            e123: Number::ONE,
        }
    }
}

impl From<Point> for Vector3 {
    fn from(value: Point) -> Self {
        Self {
            x: value.e023 / value.e123,
            y: -value.e013 / value.e123,
            z: value.e012 / value.e123,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub s: Number,
    pub e12: Number,
    pub e13: Number,
    pub e23: Number,
    pub e01: Number,
    pub e02: Number,
    pub e03: Number,
    pub e0123: Number,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct GpuTransform {
    pub s: f32,
    pub e12: f32,
    pub e13: f32,
    pub e23: f32,
    pub e01: f32,
    pub e02: f32,
    pub e03: f32,
    pub e0123: f32,
}

impl From<Transform> for GpuTransform {
    fn from(motor: Transform) -> Self {
        let Transform {
            s,
            e12,
            e13,
            e23,
            e01,
            e02,
            e03,
            e0123,
        } = motor;
        Self {
            s: s.cast(),
            e12: e12.cast(),
            e13: e13.cast(),
            e23: e23.cast(),
            e01: e01.cast(),
            e02: e02.cast(),
            e03: e03.cast(),
            e0123: e0123.cast(),
        }
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        s: Number::ONE,
        e12: Number::ZERO,
        e13: Number::ZERO,
        e23: Number::ZERO,
        e01: Number::ZERO,
        e02: Number::ZERO,
        e03: Number::ZERO,
        e0123: Number::ZERO,
    };

    pub fn translation(offset: Vector3) -> Self {
        Self {
            s: Number::ONE,
            e12: Number::ZERO,
            e13: Number::ZERO,
            e23: Number::ZERO,
            e01: offset.x * Number::from_num(-0.5),
            e02: offset.y * Number::from_num(-0.5),
            e03: offset.z * Number::from_num(-0.5),
            e0123: Number::ZERO,
        }
    }

    pub fn rotation_xy(angle: Number) -> Self {
        let (sin, cos) = sin_cos(angle * Number::from_num(0.5));
        Self {
            s: cos,
            e12: sin,
            e13: Number::ZERO,
            e23: Number::ZERO,
            e01: Number::ZERO,
            e02: Number::ZERO,
            e03: Number::ZERO,
            e0123: Number::ZERO,
        }
    }

    pub fn rotation_xz(angle: Number) -> Self {
        let (sin, cos) = sin_cos(angle * Number::from_num(0.5));
        Self {
            s: cos,
            e12: Number::ZERO,
            e13: sin,
            e23: Number::ZERO,
            e01: Number::ZERO,
            e02: Number::ZERO,
            e03: Number::ZERO,
            e0123: Number::ZERO,
        }
    }

    pub fn rotation_yz(angle: Number) -> Self {
        let (sin, cos) = sin_cos(angle * Number::from_num(0.5));
        Self {
            s: cos,
            e12: Number::ZERO,
            e13: Number::ZERO,
            e23: sin,
            e01: Number::ZERO,
            e02: Number::ZERO,
            e03: Number::ZERO,
            e0123: Number::ZERO,
        }
    }

    pub fn apply(self, other: Self) -> Self {
        let a = self.s;
        let b = self.e12;
        let c = self.e13;
        let d = self.e23;
        let e = self.e01;
        let f = self.e02;
        let g = self.e03;
        let h = self.e0123;
        let i = other.s;
        let j = other.e12;
        let k = other.e13;
        let l = other.e23;
        let m = other.e01;
        let n = other.e02;
        let o = other.e03;
        let p = other.e0123;

        /*
        Combining Motors

        (a + b*e1*e2 + c*e1*e3 + d*e2*e3 + e*e0*e1 + f*e0*e2 + g*e0*e3 + h*e0*e1*e2*e3)
        *(i + j*e1*e2 + k*e1*e3 + l*e2*e3 + m*e0*e1 + n*e0*e2 + o*e0*e3 + p*e0*e1*e2*e3)

        -1*b*j + -1*c*k + -1*d*l + a*i
        + (-1*c*l + a*j + b*i + d*k)*e1*e2
        + (-1*d*j + a*k + b*l + c*i)*e1*e3
        + (-1*b*k + a*l + c*j + d*i)*e2*e3
        + (-1*d*p + -1*f*j + -1*g*k + -1*h*l + a*m + b*n + c*o + e*i)*e0*e1
        + (-1*b*m + -1*g*l + a*n + c*p + d*o + e*j + f*i + h*k)*e0*e2
        + (-1*b*p + -1*c*m + -1*d*n + -1*h*j + a*o + e*k + f*l + g*i)*e0*e3
        + (-1*c*n + -1*f*k + a*p + b*o + d*m + e*l + g*j + h*i)*e0*e1*e2*e3
        */

        Self {
            s: -b * j + -c * k + -d * l + a * i,
            e12: -c * l + a * j + b * i + d * k,
            e13: -d * j + a * k + b * l + c * i,
            e23: -b * k + a * l + c * j + d * i,
            e01: -d * p + -f * j + -g * k + -h * l + a * m + b * n + c * o + e * i,
            e02: -b * m + -g * l + a * n + c * p + d * o + e * j + f * i + h * k,
            e03: -b * p + -c * m + -d * n + -h * j + a * o + e * k + f * l + g * i,
            e0123: -c * n + -f * k + a * p + b * o + d * m + e * l + g * j + h * i,
        }
    }

    pub fn pre_apply(self, other: Self) -> Self {
        other.apply(self)
    }

    pub fn inverse(self) -> Self {
        Self {
            s: self.s,
            e12: -self.e12,
            e13: -self.e13,
            e23: -self.e23,
            e01: -self.e01,
            e02: -self.e02,
            e03: -self.e03,
            e0123: self.e0123,
        }
    }

    pub fn magnitude_squared(self) -> Number {
        (self.apply(self.inverse())).s
    }

    pub fn magnitude(self) -> Number {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        let inverse_magnitude = self.magnitude().recip();
        let Self {
            s,
            e12,
            e13,
            e23,
            e01,
            e02,
            e03,
            e0123,
        } = self;
        Self {
            s: s * inverse_magnitude,
            e12: e12 * inverse_magnitude,
            e13: e13 * inverse_magnitude,
            e23: e23 * inverse_magnitude,
            e01: e01 * inverse_magnitude,
            e02: e02 * inverse_magnitude,
            e03: e03 * inverse_magnitude,
            e0123: e0123 * inverse_magnitude,
        }
    }
}
