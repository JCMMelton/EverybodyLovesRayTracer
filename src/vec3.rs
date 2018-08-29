use std::ops::{Neg, Div, DivAssign, Sub, SubAssign, Add, AddAssign, Mul, MulAssign};
use std::f32;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn from_u8(x: u8, y: u8, z: u8) -> Self {
        Vec3 {
            e: [x as f32, y as f32, z as f32]
        }
    }

    pub fn from_vec3(v: &Vec3) -> Self {
        Vec3 {
            e: [
                v.e[0],
                v.e[1],
                v.e[2]
            ]
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn x_u8(&self) -> u8 {
        self.e[0] as u8
    }

    pub fn y_u8(&self) -> u8 {
        self.e[1] as u8
    }

    pub fn z_u8(&self) -> u8 {
        self.e[2] as u8
    }

    pub fn r_u8(&self) -> u8 {
        self.e[0] as u8
    }

    pub fn g_u8(&self) -> u8 {
        self.e[1] as u8
    }

    pub fn b_u8(&self) -> u8 {
        self.e[2] as u8
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(f32::powf(self.e[0], 2.0) + f32::powf(self.e[1], 2.0) + f32::powf(self.e[2], 2.0))
    }

    pub fn squared_length(&self) -> f32 {
        f32::powf(self.e[0], 2.0) + f32::powf(self.e[1], 2.0) + f32::powf(self.e[2], 2.0)
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3{
            e: [
                v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],
                -(v1.e[0]*v2.e[2] - v1.e[2]*v2.e[0]),
                v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0]
            ]
        }
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        let length: f32 = v.length();
        v/length
    }

    pub fn copy(&self) -> Vec3 {
        Vec3 {
            e: [
                self.e[0],
                self.e[1],
                self.e[2]
            ]
        }
    }

}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2]
            ]
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2]
            ]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other,
                self.e[1] / other,
                self.e[2] / other
            ]
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]
            ]
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other,
                self.e[1] - other,
                self.e[2] - other
            ]
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]
            ]
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other,
                self.e[1] + other,
                self.e[2] + other
            ]
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]
            ]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other
            ]
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                other.e[0] * self,
                other.e[1] * self,
                other.e[2] * self
            ]
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                other.e[0] - self,
                other.e[1] - self,
                other.e[2] - self
            ]
        }
    }
}