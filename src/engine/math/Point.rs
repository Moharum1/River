use crate::engine::math::Vector::{Vector2, Vector3};
use std::f32;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::real::Real;
use num_traits::Signed;


#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq)]
pub(crate) struct Point2<T>{
    pub x: T,
    pub y: T,
}
impl<T> Point2<T> where T: Copy + Signed + Clone + Real{
    pub fn new(x: T, y: T) -> Self{
        Self{x, y}
    }

    pub fn from_point3(p : Point3<T>) -> Self{
        Self{x: p.x, y: p.y}
    }

    pub fn neg(&self) -> Self{
        Self{
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn abs(&self) -> Self{
        Self{
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub(crate) fn distance(&self, p2 : Point2<T>) -> T{
        Vector2{
            x: self.x - p2.x,
            y: self.y - p2.y,
        }.length()
    }

    fn distance_sq(&self, p2 : Point2<T>) -> T{
        self.distance(p2).sqrt()
    }

    fn length(&self) -> T{
        self.x * self.x + self.y * self.y
    }

    fn length_sq(&self) -> T{
        self.length().sqrt()
    }

    fn normalize(&self) -> Self{
        let len = self.length();
        Self {
            x: self.x/ len,
            y: self.y / len,
        }
    }

    pub fn min_component(&self) -> T{
        self.x.min(self.y)
    }

    pub fn max_component(&self) -> T{
        self.x.max(self.y)
    }

    pub fn max(&self, rhs : Point2<T>) -> Point2<T>{
        Self{
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    pub fn min(&self, rhs : Point2<T>) -> Point2<T>{
        Self{
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    pub fn floor(&self) -> Self{
        Self{
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(&self) -> Self{
        Self{
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    // pub fn linear_interpolate(&self, other: &Self, t:T) -> Self{
    //     (1 as T - t) * self + t * other
    // }

}
impl<T> Add<Vector2<T>> for Point2<T> where T: Add<Output = T>{
    type Output = Point2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<Point2<T>> for Point2<T> where T: Add<Output = T>{
    type Output = Point2<T>;

    fn add(self, rhs: Point2<T>) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Point2<T>> for Point2<T> where T: Sub<Output = T>{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2{
            x:  self.x - rhs.x,
            y:  self.y - rhs.y,
        }
    }
}

impl<T> Sub<Vector2<T>> for Point2<T> where T: Sub<Output = T>{
    type Output = Point2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        self - rhs
    }
}

impl<T> Mul<T> for Point2<T> where T: Mul<Output = T> + Copy{
    type Output = Point2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<f32> for Point2<T> where T: Mul<f32, Output = T>{
    type Output = Point2<T>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self{
            x: self.x * inv,
            y: self.y * inv,
        }
    }
}
#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq)]
pub(crate) struct Point3<T>{
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Point3<T> where T: Signed + Clone + Real{
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

    pub fn abs(&self) -> Self{
        Self{
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn length(&self) -> T{
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone() + self.z.clone() * self.z.clone()
    }

    fn length_sq(&self) -> T{
        self.length().sqrt()
    }

    pub(crate) fn distance(&self, p2 : Point3<T>) -> T{
        Vector3{
            x: self.x - p2.x,
            y: self.y - p2.y,
            z: self.z - p2.z
        }.length()
    }

    fn distance_sq(&self, p2 : Point3<T>) -> T{
        self.distance(p2).sqrt()
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

    pub fn max(&self, rhs : &Point3<T>) -> Point3<T> {
        Self{
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    pub fn min(&self, rhs : &Point3<T>) -> Point3<T>{
        Self{
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }
    // pub fn linear_interpolate(&self, other: &Self, t:T) -> Self{
    //     (1 as T - t) * self + t * other
    // }

    pub fn floor(&self) -> Self{
        Self{
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(&self) -> Self{
        Self{
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }
}

impl<T> Add<Vector3<T>> for Point3<T> where T: Add<Output = T>{
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<Point3<T>> for Point3<T> where T: Add<Output = T>{
    type Output = Point3<T>;

    fn add(self, rhs: Point3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub<Point3<T>> for Point3<T> where T: Sub<Output = T>{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub<Vector3<T>> for Point3<T> where T: Sub<Output = T>{
    type Output = Point3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Point3<T> where T: Mul<Output = T> + Copy{
    type Output = Point3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
        }
    }
}

impl<T> Div<f32> for Point3<T> where T: Mul<f32, Output = T>{
    type Output = Point3<T>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self{
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}


pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i32>;

#[cfg(test)]
mod tests {
    use crate::engine::math::Vector::{Vector2, Vector2f, Vector3, Vector3f};
    use super::*;

    #[test]
    fn test_point2_add_vector() {
        let p = Point2f { x: 1.0, y: 2.0 };
        let v = Vector2f { x: 3.0, y: 4.0};
        let result = p + v;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_point2_sub_point2() {
        let p1 = Point2f { x: 5.0, y: 3.0 };
        let p2 = Point2f { x: 2.0, y: 1.0 };
        let v: Vector2<f32> = p1 - p2;
        assert_eq!(v, Vector2::new(3.0, 2.0))
    }

    #[test]
    fn test_point3_add_vector() {
        let p = Point3f { x: 1.0, y: 1.0, z: 1.0 };
        let v = Vector3 { x: 2.0, y: 3.0, z: 4.0 };
        let result = p + v;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 5.0);
    }

    // #[test]
    // fn test_point3_distance() {
    //     let p1 = Point3f::new(1.0, 2.0, 3.0);
    //     let p2 = Point3f::new(4.0, 6.0, 3.0);
    //     let dist = p1.distance(p2);
    //     assert_eq!(dist, Vector3f{
    //         x: -3.0,
    //         y: -4.0,
    //         z: 0.0,
    //     });
    // }


    #[test]
    fn test_point3_min_max_component() {
        let p = Point3f::new(3.0, -1.0, 7.0);
        assert_eq!(p.min_component(), -1.0);
        assert_eq!(p.max_component(), 7.0);
    }

    #[test]
    fn test_point3_min_max_with_point() {
        let a = Point3f::new(1.0, 5.0, 3.0);
        let b = Point3f::new(2.0, 3.0, 4.0);
        let min = a.min(&b);
        let max = a.max(&b);
        assert_eq!(min, Point3f::new(1.0, 3.0, 3.0));
        assert_eq!(max, Point3f::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_point2_floor_ceil() {
        let p = Point2f::new(1.7, -2.3);
        assert_eq!(p.floor(), Point2f::new(1.0, -3.0));
        assert_eq!(p.ceil(), Point2f::new(2.0, -2.0));
    }

    #[test]
    fn test_point3_floor_ceil() {
        let p = Point3f::new(1.7, -2.3, 0.9);
        assert_eq!(p.floor(), Point3f::new(1.0, -3.0, 0.0));
        assert_eq!(p.ceil(), Point3f::new(2.0, -2.0, 1.0));
    }
}