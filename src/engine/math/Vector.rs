use std::f32;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::real::Real;
use num_traits::Signed;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Vector2<T>{
    pub x: T,
    pub y: T,
}
impl<T> Vector2<T> where T: Signed + Clone + Real{
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

    pub fn dot(&self, rhs : Vector2<T>) -> T {
        self.x.clone() * rhs.x + self.y.clone() * rhs.y
    }

    pub fn abs_dot(&self, rhs : Vector2<T>) -> T {
        self.dot(rhs).abs()
    }

    pub(crate) fn length(&self) -> T{
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()
    }

    fn length_sq(&self) -> T{
        self.length().sqrt()
    }

    fn normalize(&self) -> Self{
        let len = self.length();
        Self {
            x: self.x.clone() / len.clone(),
            y: self.y.clone() / len,
        }
    }

    pub fn min_component(&self) -> T{
        self.x.min(self.y)
    }

    pub fn max_component(&self) -> T{
        self.x.max(self.y)
    }

    pub fn max(&self, rhs : Vector2<T>) -> Vector2<T>{
        Self{
            x: self.x.clone().max(rhs.x.clone()),
            y: self.y.clone().max(rhs.y.clone()),
        }
    }

    pub fn min(&self, rhs : Vector2<T>) -> Vector2<T>{
        Self{
            x: self.x.clone().min(rhs.x.clone()),
            y: self.y.clone().min(rhs.y.clone()),
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
impl<T> Vector3<T> where T: Signed + Clone + Real{
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

    pub fn dot(&self, rhs : &Vector3<T>) -> T {
        self.x.clone() * rhs.x + self.y.clone() * rhs.y + self.z.clone() * rhs.z
    }

    pub fn abs_dot(&self, rhs : &Vector3<T>) -> T {
        self.dot(rhs).abs()
    }

    pub fn cross(&self, rhs : &Vector3<T>) -> Self{
        Self{
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z.clone() * rhs.x.clone() - self.x.clone() * rhs.z.clone(),
            z: self.x.clone() * rhs.y.clone() - self.y.clone() * rhs.x.clone(),
        }
    }

    pub fn co_ordinate_system(&self) -> (Vector3<T>, Vector3<T>){
        let v2;
        let v3;
        if (self.x.abs() > self.y.abs()){
            let div = (self.x * self.x + self.z * self.z).sqrt();
            v2 = Vector3{
                x : -self.z.clone() / div,
                y : T::zero(),
                z : self.x.clone() / div
            }
        }else {
            let div = (self.y * self.y + self.z * self.z).sqrt();
            v2 = Vector3{
                x : T::zero(),
                y : self.z.clone() / div,
                z : -self.y.clone() / div
            }
        }

        v3 = self.cross(&v2);

        (v2, v3)
    }

    pub(crate) fn length(&self) -> T{
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone() + self.z.clone() * self.z.clone()
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

    pub fn max(&self, rhs : Vector3<T>) -> Vector3<T> {
        Self{
            x: self.x.clone().max(rhs.x.clone()),
            y: self.y.clone().max(rhs.y.clone()),
            z: self.z.clone().max(rhs.z.clone()),
        }
    }

    pub fn min(&self, rhs : Vector3<T>) -> Vector3<T>{
        Self{
            x: self.x.clone().min(rhs.x.clone()),
            y: self.y.clone().min(rhs.y.clone()),
            z: self.z.clone().min(rhs.z.clone()),
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


pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i32>;
pub type Vector3f = Vector3<f32>;
pub type Vector3i = Vector3<i32>;