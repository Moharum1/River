use std::ops::{Add, Div, Mul, Sub};
use num_traits::real::Real;
use num_traits::Signed;
use crate::engine::math::Vector::Vector3;

/** A normal is a Vector that is perpendicular to a surface at a certain position
It can also be defined as the cross-product between tow no-parallel vector
*/
#[derive(Debug, Clone, Copy, Default)]
pub struct Normal3<T>{
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Normal3f = Normal3<f32>;

impl<T> Normal3<T> where T: Signed + Clone + Real{
    pub fn new(x: T, y: T,z : T) -> Self{
        Self{x, y, z}
    }

    pub fn neg(&self) -> Self{
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub(crate) fn length(&self) -> T{
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length_sq(&self) -> T{
        self.length().sqrt()
    }

    pub fn normalize(&self) -> Self{
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn min_component(&self) -> T{
        self.x.min(self.y.min(self.z))
    }

    pub fn max_component(&self) -> T{
        self.x.max(self.y.max(self.z))
    }

    pub fn dot(&self, rhs : &Vector3<T>) -> T {
        self.x.clone() * rhs.x + self.y.clone() * rhs.y + self.z.clone() * rhs.z
    }
    pub fn abs_dot(&self, rhs : &Vector3<T>) -> T {
        self.dot(rhs).abs()
    }

    pub fn face_forward(&self, v : &Vector3<T>) -> Self{
        if self.dot(&v) < T::zero(){
            self.neg()
        }else {
            self.clone()
        }
    }
}

impl<T> Add for Normal3<T> where T: Add<Output = T>{
    type Output = Normal3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Normal3<T> where T: Sub<Output = T>{
    type Output = Normal3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<f32> for Normal3<T> where T: Mul<f32, Output = T>{
    type Output = Normal3<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<f32> for Normal3<T> where T: Mul<f32, Output = T>{
    type Output = Normal3<T>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self{
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}