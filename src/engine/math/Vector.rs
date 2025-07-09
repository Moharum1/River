use std::f32;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::Signed;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Vector2<T>{
    pub x: T,
    pub y: T,
}
impl<T> Vector2<T> where T: Signed + Clone{
    pub fn new(x: T, y: T) -> Self{
        Self{x, y}
    }

    pub fn neg(&self) -> Self{
        Self{
            x: -self.x.clone(),
            y: -self.y.clone(),
        }
    }

    pub fn abs(&self) -> Self{
        Self{
            x: self.x.clone().abs(),
            y: self.y.clone().abs(),
        }
    }
}
impl<T> Add for Vector2<T> where T: Add<Output = T>{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector2<T> where T: Sub<Output = T>{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<f32> for Vector2<T> where T: Mul<f32, Output = T>{
    type Output = Vector2<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<f32> for Vector2<T> where T: Mul<f32, Output = T>{
    type Output = Vector2<T>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self{
            x: self.x * inv,
            y: self.y * inv,
        }
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Vector3<T>{
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Vector3<T> where T: Signed + Clone{
    pub fn new(x: T, y: T,z : T) -> Self{
        Self{x, y, z}
    }

    pub fn neg(&self) -> Self{
        Self{
            x: -self.x.clone(),
            y: -self.y.clone(),
            z: -self.z.clone(),
        }
    }

    pub fn abs(&self) -> Self{
        Self{
            x: self.x.clone().abs(),
            y: self.y.clone().abs(),
            z: self.z.clone().abs(),
        }
    }
}

impl<T> Add for Vector3<T> where T: Add<Output = T>{
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vector3<T> where T: Sub<Output = T>{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<f32> for Vector3<T> where T: Mul<f32, Output = T>{
    type Output = Vector3<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<f32> for Vector3<T> where T: Mul<f32, Output = T>{
    type Output = Vector3<T>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self{
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}


type Vector2f = Vector2<f32>;
type Vector2i = Vector2<i32>;
type Vector3f = Vector3<f32>;
type Vector3i = Vector3<i32>;