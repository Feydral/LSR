#![allow(dead_code)]

use std::ops::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub const ZERO: Float2 = Float2::new(0.0, 0.0);
    pub const ONE: Float2 = Float2::new(1.0, 1.0);
    pub const TWO: Float2 = Float2::new(2.0, 2.0);
    pub const HALF: Float2 = Float2::new(0.5, 0.5);
    pub const UNIT_X: Float2 = Float2::new(1.0, 0.0);
    pub const UNIT_Y: Float2 = Float2::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Float2 {
        Float2 { x, y }
    }

    pub const fn splat(v: f32) -> Float2 {
        Float2 { x: v, y: v }
    }

    pub fn xxx(self) -> Float3 { Float3::new(self.x, self.x, self.x) }
    pub fn xxy(self) -> Float3 { Float3::new(self.x, self.x, self.y) }
    pub fn xyx(self) -> Float3 { Float3::new(self.x, self.y, self.x) }
    pub fn xyy(self) -> Float3 { Float3::new(self.x, self.y, self.y) }
    pub fn yxx(self) -> Float3 { Float3::new(self.y, self.x, self.x) }
    pub fn yxy(self) -> Float3 { Float3::new(self.y, self.x, self.y) }
    pub fn yyx(self) -> Float3 { Float3::new(self.y, self.y, self.x) }
    pub fn yyy(self) -> Float3 { Float3::new(self.y, self.y, self.y) }

    pub fn xxxx(self) -> Float4 { Float4::new(self.x, self.x, self.x, self.x) }
    pub fn xxyy(self) -> Float4 { Float4::new(self.x, self.x, self.y, self.y) }
    pub fn xyxy(self) -> Float4 { Float4::new(self.x, self.y, self.x, self.y) }
    pub fn xyyx(self) -> Float4 { Float4::new(self.x, self.y, self.y, self.x) }
    pub fn yyyy(self) -> Float4 { Float4::new(self.y, self.y, self.y, self.y) }

    #[inline(always)]
    pub fn dot(self, rhs: Float2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline(always)]
    pub fn cross(self, rhs: Float2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Float2 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    #[inline(always)]
    pub fn lerp(self, rhs: Float2, t: f32) -> Float2 {
        self + (rhs - self) * t
    }

    #[inline(always)]
    pub fn min(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x.min(rhs.x), y: self.y.min(rhs.y) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x.max(rhs.x), y: self.y.max(rhs.y) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Float2, max: Float2) -> Float2 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Float2 {
        Float2 { x: self.x.abs(), y: self.y.abs() }
    }

    #[inline(always)]
    pub fn floor(self) -> Float2 {
        Float2 { x: self.x.floor(), y: self.y.floor() }
    }

    #[inline(always)]
    pub fn ceil(self) -> Float2 {
        Float2 { x: self.x.ceil(), y: self.y.ceil() }
    }

    #[inline(always)]
    pub fn round(self) -> Float2 {
        Float2 { x: self.x.round(), y: self.y.round() }
    }

    #[inline(always)]
    pub fn fract(self) -> Float2 {
        Float2 { x: self.x.fract(), y: self.y.fract() }
    }

    #[inline(always)]
    pub fn min_component(self) -> f32 {
        self.x.min(self.y)
    }

    #[inline(always)]
    pub fn max_component(self) -> f32 {
        self.x.max(self.y)
    }
}

impl Neg for Float2 {
    type Output = Float2;
    fn neg(self) -> Float2 {
        Float2 { x: -self.x, y: -self.y }
    }
}

impl Add<Float2> for Float2 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<f32> for Float2 {
    type Output = Float2;
    fn add(self, rhs: f32) -> Float2 {
        Float2 { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<Float2> for f32 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Float2 {
        Float2 { x: self + rhs.x, y: self + rhs.y }
    }
}
impl AddAssign<Float2> for Float2 {
    fn add_assign(&mut self, rhs: Float2) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<f32> for Float2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs;
    }
}

impl Sub<Float2> for Float2 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Sub<f32> for Float2 {
    type Output = Float2;
    fn sub(self, rhs: f32) -> Float2 {
        Float2 { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<Float2> for f32 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Float2 {
        Float2 { x: self - rhs.x, y: self - rhs.y }
    }
}
impl SubAssign<Float2> for Float2 {
    fn sub_assign(&mut self, rhs: Float2) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<f32> for Float2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs;
    }
}

impl Mul<Float2> for Float2 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
impl Mul<f32> for Float2 {
    type Output = Float2;
    fn mul(self, rhs: f32) -> Float2 {
        Float2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<Float2> for f32 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Float2 {
        Float2 { x: self * rhs.x, y: self * rhs.y }
    }
}
impl MulAssign<Float2> for Float2 {
    fn mul_assign(&mut self, rhs: Float2) {
        self.x *= rhs.x; self.y *= rhs.y;
    }
}
impl MulAssign<f32> for Float2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs;
    }
}

impl Div<Float2> for Float2 {
    type Output = Float2;
    fn div(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}
impl Div<f32> for Float2 {
    type Output = Float2;
    fn div(self, rhs: f32) -> Float2 {
        Float2 { x: self.x / rhs, y: self.y / rhs }
    }
}
impl Div<Float2> for f32 {
    type Output = Float2;
    fn div(self, rhs: Float2) -> Float2 {
        Float2 { x: self / rhs.x, y: self / rhs.y }
    }
}
impl DivAssign<Float2> for Float2 {
    fn div_assign(&mut self, rhs: Float2) {
        self.x /= rhs.x; self.y /= rhs.y;
    }
}
impl DivAssign<f32> for Float2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs;
    }
}

impl From<(f32, f32)> for Float2 {
    fn from((x, y): (f32, f32)) -> Float2 {
        Float2 { x, y }
    }
}
impl From<[f32; 2]> for Float2 {
    fn from([x, y]: [f32; 2]) -> Float2 {
        Float2 { x, y }
    }
}
impl From<Float2> for [f32; 2] {
    fn from(v: Float2) -> [f32; 2] {
        [v.x, v.y]
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    pub const ZERO: Float3 = Float3::new(0.0, 0.0, 0.0);
    pub const ONE: Float3 = Float3::new(1.0, 1.0, 1.0);
    pub const TWO: Float3 = Float3::new(2.0, 2.0, 2.0);
    pub const HALF: Float3 = Float3::new(0.5, 0.5, 0.5);
    pub const UNIT_X: Float3 = Float3::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Float3 = Float3::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Float3 = Float3::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Float3 {
        Float3 { x, y, z }
    }

    pub const fn splat(v: f32) -> Float3 {
        Float3 { x: v, y: v, z: v }
    }

    pub fn from2(xy: Float2, z: f32) -> Float3 {
        Float3 { x: xy.x, y: xy.y, z }
    }

    pub fn xx(self) -> Float2 { Float2::new(self.x, self.x) }
    pub fn xy(self) -> Float2 { Float2::new(self.x, self.y) }
    pub fn xz(self) -> Float2 { Float2::new(self.x, self.z) }
    pub fn yx(self) -> Float2 { Float2::new(self.y, self.x) }
    pub fn yy(self) -> Float2 { Float2::new(self.y, self.y) }
    pub fn yz(self) -> Float2 { Float2::new(self.y, self.z) }
    pub fn zx(self) -> Float2 { Float2::new(self.z, self.x) }
    pub fn zy(self) -> Float2 { Float2::new(self.z, self.y) }
    pub fn zz(self) -> Float2 { Float2::new(self.z, self.z) }

    pub fn xxxx(self) -> Float4 { Float4::new(self.x, self.x, self.x, self.x) }
    pub fn xyzx(self) -> Float4 { Float4::new(self.x, self.y, self.z, self.x) }
    pub fn xyzy(self) -> Float4 { Float4::new(self.x, self.y, self.z, self.y) }
    pub fn xyzz(self) -> Float4 { Float4::new(self.x, self.y, self.z, self.z) }
    pub fn xyzw(self, w: f32) -> Float4 { Float4::new(self.x, self.y, self.z, w) }

    #[inline(always)]
    pub fn dot(self, rhs: Float3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    pub fn cross(self, rhs: Float3) -> Float3 {
        Float3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline(always)]
    pub fn determinant(self, b: Float3, c: Float3) -> f32 {
        self.dot(b.cross(c))
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Float3 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    #[inline(always)]
    pub fn lerp(self, rhs: Float3, t: f32) -> Float3 {
        self + (rhs - self) * t
    }

    #[inline(always)]
    pub fn reflect(self, normal: Float3) -> Float3 {
        self - normal * (2.0 * self.dot(normal))
    }

    #[inline(always)]
    pub fn min(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Float3, max: Float3) -> Float3 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Float3 {
        Float3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    #[inline(always)]
    pub fn floor(self) -> Float3 {
        Float3 { x: self.x.floor(), y: self.y.floor(), z: self.z.floor() }
    }

    #[inline(always)]
    pub fn ceil(self) -> Float3 {
        Float3 { x: self.x.ceil(), y: self.y.ceil(), z: self.z.ceil() }
    }

    #[inline(always)]
    pub fn round(self) -> Float3 {
        Float3 { x: self.x.round(), y: self.y.round(), z: self.z.round() }
    }

    #[inline(always)]
    pub fn fract(self) -> Float3 {
        Float3 { x: self.x.fract(), y: self.y.fract(), z: self.z.fract() }
    }

    #[inline(always)]
    pub fn min_component(self) -> f32 {
        self.x.min(self.y).min(self.z)
    }

    #[inline(always)]
    pub fn max_component(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }
}

impl Neg for Float3 {
    type Output = Float3;
    fn neg(self) -> Float3 {
        Float3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add<Float3> for Float3 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<f32> for Float3 {
    type Output = Float3;
    fn add(self, rhs: f32) -> Float3 {
        Float3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<Float3> for f32 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Float3 {
        Float3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}
impl AddAssign<Float3> for Float3 {
    fn add_assign(&mut self, rhs: Float3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<f32> for Float3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

impl Sub<Float3> for Float3 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<f32> for Float3 {
    type Output = Float3;
    fn sub(self, rhs: f32) -> Float3 {
        Float3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<Float3> for f32 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Float3 {
        Float3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}
impl SubAssign<Float3> for Float3 {
    fn sub_assign(&mut self, rhs: Float3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<f32> for Float3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

impl Mul<Float3> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<f32> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: f32) -> Float3 {
        Float3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Float3> for f32 {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Float3 {
        Float3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}
impl MulAssign<Float3> for Float3 {
    fn mul_assign(&mut self, rhs: Float3) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z;
    }
}
impl MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

impl Div<Float3> for Float3 {
    type Output = Float3;
    fn div(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<f32> for Float3 {
    type Output = Float3;
    fn div(self, rhs: f32) -> Float3 {
        Float3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl Div<Float3> for f32 {
    type Output = Float3;
    fn div(self, rhs: Float3) -> Float3 {
        Float3 { x: self / rhs.x, y: self / rhs.y, z: self / rhs.z }
    }
}
impl DivAssign<Float3> for Float3 {
    fn div_assign(&mut self, rhs: Float3) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z;
    }
}
impl DivAssign<f32> for Float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}

impl From<(f32, f32, f32)> for Float3 {
    fn from((x, y, z): (f32, f32, f32)) -> Float3 {
        Float3 { x, y, z }
    }
}
impl From<[f32; 3]> for Float3 {
    fn from([x, y, z]: [f32; 3]) -> Float3 {
        Float3 { x, y, z }
    }
}
impl From<Float3> for [f32; 3] {
    fn from(v: Float3) -> [f32; 3] {
        [v.x, v.y, v.z]
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {
    pub const ZERO: Float4 = Float4::new(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Float4 = Float4::new(1.0, 1.0, 1.0, 1.0);
    pub const TWO: Float4 = Float4::new(2.0, 2.0, 2.0, 2.0);
    pub const HALF: Float4 = Float4::new(0.5, 0.5, 0.5, 0.5);
    pub const UNIT_X: Float4 = Float4::new(1.0, 0.0, 0.0, 0.0);
    pub const UNIT_Y: Float4 = Float4::new(0.0, 1.0, 0.0, 0.0);
    pub const UNIT_Z: Float4 = Float4::new(0.0, 0.0, 1.0, 0.0);
    pub const UNIT_W: Float4 = Float4::new(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Float4 {
        Float4 { x, y, z, w }
    }

    pub const fn splat(v: f32) -> Float4 {
        Float4 { x: v, y: v, z: v, w: v }
    }

    pub fn from2(xy: Float2, z: f32, w: f32) -> Float4 {
        Float4 { x: xy.x, y: xy.y, z, w }
    }

    pub fn from3(xyz: Float3, w: f32) -> Float4 {
        Float4 { x: xyz.x, y: xyz.y, z: xyz.z, w }
    }

    pub fn xx(self) -> Float2 { Float2::new(self.x, self.x) }
    pub fn xy(self) -> Float2 { Float2::new(self.x, self.y) }
    pub fn xz(self) -> Float2 { Float2::new(self.x, self.z) }
    pub fn xw(self) -> Float2 { Float2::new(self.x, self.w) }
    pub fn yx(self) -> Float2 { Float2::new(self.y, self.x) }
    pub fn yy(self) -> Float2 { Float2::new(self.y, self.y) }
    pub fn yz(self) -> Float2 { Float2::new(self.y, self.z) }
    pub fn yw(self) -> Float2 { Float2::new(self.y, self.w) }
    pub fn zx(self) -> Float2 { Float2::new(self.z, self.x) }
    pub fn zy(self) -> Float2 { Float2::new(self.z, self.y) }
    pub fn zz(self) -> Float2 { Float2::new(self.z, self.z) }
    pub fn zw(self) -> Float2 { Float2::new(self.z, self.w) }
    pub fn wx(self) -> Float2 { Float2::new(self.w, self.x) }
    pub fn wy(self) -> Float2 { Float2::new(self.w, self.y) }
    pub fn wz(self) -> Float2 { Float2::new(self.w, self.z) }
    pub fn ww(self) -> Float2 { Float2::new(self.w, self.w) }

    pub fn xyz(self) -> Float3 { Float3::new(self.x, self.y, self.z) }
    pub fn xyw(self) -> Float3 { Float3::new(self.x, self.y, self.w) }
    pub fn xzy(self) -> Float3 { Float3::new(self.x, self.z, self.y) }
    pub fn xzw(self) -> Float3 { Float3::new(self.x, self.z, self.w) }
    pub fn xwy(self) -> Float3 { Float3::new(self.x, self.w, self.y) }
    pub fn xwz(self) -> Float3 { Float3::new(self.x, self.w, self.z) }
    pub fn yxz(self) -> Float3 { Float3::new(self.y, self.x, self.z) }
    pub fn yxw(self) -> Float3 { Float3::new(self.y, self.x, self.w) }
    pub fn yzx(self) -> Float3 { Float3::new(self.y, self.z, self.x) }
    pub fn yzw(self) -> Float3 { Float3::new(self.y, self.z, self.w) }
    pub fn ywx(self) -> Float3 { Float3::new(self.y, self.w, self.x) }
    pub fn ywz(self) -> Float3 { Float3::new(self.y, self.w, self.z) }
    pub fn zxy(self) -> Float3 { Float3::new(self.z, self.x, self.y) }
    pub fn zxw(self) -> Float3 { Float3::new(self.z, self.x, self.w) }
    pub fn zyx(self) -> Float3 { Float3::new(self.z, self.y, self.x) }
    pub fn zyw(self) -> Float3 { Float3::new(self.z, self.y, self.w) }
    pub fn zwx(self) -> Float3 { Float3::new(self.z, self.w, self.x) }
    pub fn zwy(self) -> Float3 { Float3::new(self.z, self.w, self.y) }
    pub fn wxy(self) -> Float3 { Float3::new(self.w, self.x, self.y) }
    pub fn wxz(self) -> Float3 { Float3::new(self.w, self.x, self.z) }
    pub fn wyx(self) -> Float3 { Float3::new(self.w, self.y, self.x) }
    pub fn wyz(self) -> Float3 { Float3::new(self.w, self.y, self.z) }
    pub fn wzx(self) -> Float3 { Float3::new(self.w, self.z, self.x) }
    pub fn wzy(self) -> Float3 { Float3::new(self.w, self.z, self.y) }

    #[inline(always)]
    pub fn dot(self, rhs: Float4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Float4 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    #[inline(always)]
    pub fn lerp(self, rhs: Float4, t: f32) -> Float4 {
        self + (rhs - self) * t
    }

    #[inline(always)]
    pub fn min(self, rhs: Float4) -> Float4 {
        Float4 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
            w: self.w.min(rhs.w),
        }
    }

    #[inline(always)]
    pub fn max(self, rhs: Float4) -> Float4 {
        Float4 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
            w: self.w.max(rhs.w),
        }
    }

    #[inline(always)]
    pub fn clamp(self, min: Float4, max: Float4) -> Float4 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Float4 {
        Float4 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs(), w: self.w.abs() }
    }

    #[inline(always)]
    pub fn floor(self) -> Float4 {
        Float4 { x: self.x.floor(), y: self.y.floor(), z: self.z.floor(), w: self.w.floor() }
    }

    #[inline(always)]
    pub fn ceil(self) -> Float4 {
        Float4 { x: self.x.ceil(), y: self.y.ceil(), z: self.z.ceil(), w: self.w.ceil() }
    }

    #[inline(always)]
    pub fn round(self) -> Float4 {
        Float4 { x: self.x.round(), y: self.y.round(), z: self.z.round(), w: self.w.round() }
    }

    #[inline(always)]
    pub fn fract(self) -> Float4 {
        Float4 { x: self.x.fract(), y: self.y.fract(), z: self.z.fract(), w: self.w.fract() }
    }

    #[inline(always)]
    pub fn min_component(self) -> f32 {
        self.x.min(self.y).min(self.z).min(self.w)
    }

    #[inline(always)]
    pub fn max_component(self) -> f32 {
        self.x.max(self.y).max(self.z).max(self.w)
    }
}

impl Neg for Float4 {
    type Output = Float4;
    fn neg(self) -> Float4 {
        Float4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl Add<Float4> for Float4 {
    type Output = Float4;
    fn add(self, rhs: Float4) -> Float4 {
        Float4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}
impl Add<f32> for Float4 {
    type Output = Float4;
    fn add(self, rhs: f32) -> Float4 {
        Float4 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs }
    }
}
impl Add<Float4> for f32 {
    type Output = Float4;
    fn add(self, rhs: Float4) -> Float4 {
        Float4 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z, w: self + rhs.w }
    }
}
impl AddAssign<Float4> for Float4 {
    fn add_assign(&mut self, rhs: Float4) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w;
    }
}
impl AddAssign<f32> for Float4 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs; self.z += rhs; self.w += rhs;
    }
}

impl Sub<Float4> for Float4 {
    type Output = Float4;
    fn sub(self, rhs: Float4) -> Float4 {
        Float4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}
impl Sub<f32> for Float4 {
    type Output = Float4;
    fn sub(self, rhs: f32) -> Float4 {
        Float4 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}
impl Sub<Float4> for f32 {
    type Output = Float4;
    fn sub(self, rhs: Float4) -> Float4 {
        Float4 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z, w: self - rhs.w }
    }
}
impl SubAssign<Float4> for Float4 {
    fn sub_assign(&mut self, rhs: Float4) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; self.w -= rhs.w;
    }
}
impl SubAssign<f32> for Float4 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs; self.w -= rhs;
    }
}

impl Mul<Float4> for Float4 {
    type Output = Float4;
    fn mul(self, rhs: Float4) -> Float4 {
        Float4 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}
impl Mul<f32> for Float4 {
    type Output = Float4;
    fn mul(self, rhs: f32) -> Float4 {
        Float4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}
impl Mul<Float4> for f32 {
    type Output = Float4;
    fn mul(self, rhs: Float4) -> Float4 {
        Float4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}
impl MulAssign<Float4> for Float4 {
    fn mul_assign(&mut self, rhs: Float4) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z; self.w *= rhs.w;
    }
}
impl MulAssign<f32> for Float4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs; self.w *= rhs;
    }
}

impl Div<Float4> for Float4 {
    type Output = Float4;
    fn div(self, rhs: Float4) -> Float4 {
        Float4 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}
impl Div<f32> for Float4 {
    type Output = Float4;
    fn div(self, rhs: f32) -> Float4 {
        Float4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}
impl Div<Float4> for f32 {
    type Output = Float4;
    fn div(self, rhs: Float4) -> Float4 {
        Float4 { x: self / rhs.x, y: self / rhs.y, z: self / rhs.z, w: self / rhs.w }
    }
}
impl DivAssign<Float4> for Float4 {
    fn div_assign(&mut self, rhs: Float4) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z; self.w /= rhs.w;
    }
}
impl DivAssign<f32> for Float4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs; self.w /= rhs;
    }
}

impl From<(f32, f32, f32, f32)> for Float4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Float4 {
        Float4 { x, y, z, w }
    }
}
impl From<[f32; 4]> for Float4 {
    fn from([x, y, z, w]: [f32; 4]) -> Float4 {
        Float4 { x, y, z, w }
    }
}
impl From<Float4> for [f32; 4] {
    fn from(v: Float4) -> [f32; 4] {
        [v.x, v.y, v.z, v.w]
    }
}
