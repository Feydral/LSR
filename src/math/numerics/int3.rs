use std::ops::*;

use crate::math::numerics::uint3::UInt3;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Int3 {
    pub const fn new(x: i32, y: i32, z: i32) -> Int3 {
        Int3 { x, y, z }
    }

    pub fn div_euclid(self, rhs: i32) -> Int3 {
        Int3 {
            x: self.x.div_euclid(rhs),
            y: self.y.div_euclid(rhs),
            z: self.z.div_euclid(rhs),
        }
    }
    
    pub fn rem_euclid(self, rhs: i32) -> Int3 {
        Int3 {
            x: self.x.rem_euclid(rhs),
            y: self.y.rem_euclid(rhs),
            z: self.z.rem_euclid(rhs),
        }
    }

    pub fn to_uint3(self) -> UInt3 {
        UInt3 {
            x: self.x as u32,
            y: self.y as u32,
            z: self.z as u32,
        }
    }
}

// ======= ADD ======= 
impl Add for Int3 {
    type Output = Int3;
    fn add(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<i32> for Int3 {
    type Output = Int3;
    fn add(self, rhs: i32) -> Int3 {
        Int3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<Int3> for i32 {
    type Output = Int3;
    fn add(self, rhs: Int3) -> Int3 {
        Int3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}

impl AddAssign for Int3 {
    fn add_assign(&mut self, rhs: Int3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<i32> for Int3 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

// ======= SUB ======= 
impl Sub for Int3 {
    type Output = Int3;
    fn sub(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<i32> for Int3 {
    type Output = Int3;
    fn sub(self, rhs: i32) -> Int3 {
        Int3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<Int3> for i32 {
    type Output = Int3;
    fn sub(self, rhs: Int3) -> Int3 {
        Int3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}

impl SubAssign for Int3 {
    fn sub_assign(&mut self, rhs: Int3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<i32> for Int3 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

// ======= MUL =======
impl Mul<i32> for Int3 {
    type Output = Int3;
    fn mul(self, rhs: i32) -> Int3 {
        Int3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Int3> for i32 {
    type Output = Int3;
    fn mul(self, rhs: Int3) -> Int3 {
        Int3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl MulAssign<i32> for Int3 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

// ======= DIV =======
impl Div<i32> for Int3 {
    type Output = Int3;
    fn div(self, rhs: i32) -> Int3 {
        Int3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl DivAssign<i32> for Int3 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}

// ======= REM =======
impl Rem<i32> for Int3 {
    type Output = Int3;
    fn rem(self, rhs: i32) -> Int3 {
        Int3 { x: self.x % rhs, y: self.y % rhs, z: self.z % rhs }
    }
}

impl RemAssign<i32> for Int3 {
    fn rem_assign(&mut self, rhs: i32) {
        self.x %= rhs; self.y %= rhs; self.z %= rhs;
    }
}