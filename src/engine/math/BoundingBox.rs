use std::ops::{Div, Index};
use num_traits::{Bounded, FromPrimitive, Signed};
use crate::engine::math::lerp;
use crate::engine::math::Point::{Point2, Point3, Point3f};
use crate::engine::math::Vector::{Vector2, Vector3};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Bound2<T>{
    pub p_min : Point2<T>,
    pub p_max : Point2<T>
}

impl<T> Bound2<T>
where T: Signed + Bounded + Copy + num_traits::Float + FromPrimitive
{
    pub fn new() -> Self{
        let min=  <T as Bounded>::min_value();
        let max=  <T as Bounded>::max_value();
        Self{
            p_min: Point2::new(max,max),
            p_max: Point2::new(min,min),
        }
    }

    pub fn from_point(p : &Point2<T>) -> Self{
        Self{
            p_min: p.clone(),
            p_max: p.clone(),
        }
    }

    pub fn from_points(p1 : &Point2<T>, p2 : &Point2<T>) -> Self{
        Self{
            p_min: Point2{
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            p_max: Point2 {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        }
    }

    pub fn corner(&self, corner : usize) -> Point2<T> {
        Point2{
            x: if (corner & 1) != 0 { self[1].x } else { self[0].x },
            y: if (corner & 2) != 0 { self[1].y } else { self[0].y },
        }
    }

    pub fn union(&self, other: &Point2<T>) -> Self {
        Bound2{
            p_min: Point2 {
                x: self.p_min.x.min(other.x),
                y: self.p_min.y.min(other.y),
            },
            p_max: Point2 {
                x: self.p_max.x.max(other.x),
                y: self.p_max.y.max(other.y),
            },
        }
    }

    pub fn union_with_box(&self, other : Bound2<T>) -> Self {
        Bound2{
            p_min: Point2 {
                x: self.p_min.x.min(other.p_min.x),
                y: self.p_min.y.min(other.p_min.y),
            },
            p_max: Point2 {
                x: self.p_max.x.max(other.p_max.x),
                y: self.p_max.y.max(other.p_max.y),
            },
        }
    }

    pub fn intersection(&self, other : Bound2<T>) -> Self {
        Bound2{
            p_min: Point2 {
                x: self.p_min.x.max(other.p_min.x),
                y: self.p_min.y.max(other.p_min.y),
            },
            p_max: Point2 {
                x: self.p_max.x.min(other.p_max.x),
                y: self.p_max.y.min(other.p_max.y)
            },
        }
    }

    pub fn overlaps(&self, other : Bound2<T>) -> bool{
        let x =  self.p_max.x >= other.p_min.x && self.p_min.x <= other.p_max.x;
        let y =  self.p_max.y >= other.p_min.y && self.p_min.y <= other.p_max.y;

        x && y
    }

    pub fn inside(&self, p : Point2<T>) -> bool {
                p.x >= self.p_min.x && p.x <= self.p_max.x
            &&  p.y >= self.p_min.y && p.y <= self.p_max.y

    }

    /// A variant of `inside()` that doesn't consider points
    /// In the upper boundary to be inside the boundary
    pub fn inside_exclusive(&self, p : Point2<T>) -> bool {
                p.x >= self.p_min.x && p.x < self.p_max.x
            &&  p.y >= self.p_min.y && p.y < self.p_max.y

    }

    pub fn expands(&self, delta : T) -> Self {
        Bound2{
            p_min: self.p_min - Vector2::new(delta, delta),
            p_max: self.p_max + Vector2::new(delta, delta),
        }
    }

    /// Return a Vector along the box diagonal from the min point to max point
    pub fn diagonal(&self) -> Vector2<T>{
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> T{
        let d = self.diagonal();
        T::from_u8(2).unwrap() * (d.x * d.x + d.y * d.y)
    }

    pub fn volume(&self) -> T{
        let d = self.diagonal();
        d.x * d.y
    }

    pub fn max_extent(&self) -> i32 {
        let d = self.diagonal();

        if (d.x > d.y){
            0
        }else{
            1
        }
    }

    /// Compute the linear interpolation
    /// between the corners of the box by the given amount in each dimension.
    pub fn lerp(&self, p : &Point2<T>) -> Point2<T> {
        Point2{
            x: lerp(p.x, self.p_min.x, self.p_max.x),
            y: lerp(p.y, self.p_min.y, self.p_max.y),
        }
    }

    pub fn offset(&self, p : Point2<T>) -> Vector2<T>{
        let mut o = p - self.p_min;
        if (self.p_max.x > self.p_min.x){
            o.x = o.x / (self.p_max.x - self.p_min.x);
        }
        if (self.p_max.y > self.p_min.y){
            o.y = o.y / (self.p_max.y - self.p_min.y);
        }

        o
    }

    /// Return the Origin Point and the radius of a sphere that bounds the bounding box
    pub fn bounding_sphere(&self) -> (Point2<T>, T){
        let center =  (self.p_min + self.p_max) * T::from_f32(0.5).unwrap() ;
        let radius = if (self.inside(center)){
            self.p_max.distance(center)
        }else { T::from_f32(0.0).unwrap() };

        (center, radius)
    }
}

impl<T> Index<usize> for Bound2<T>{
    type Output = Point2<T>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("Index out of bounds for Vector3: {}", index),
        }
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Bound3<T>{
    pub p_min : Point3<T>,
    pub p_max : Point3<T>
}

impl<T> Bound3<T>
where T: Signed + Bounded + Copy + num_traits::Float + FromPrimitive
{
    pub fn new() -> Self{
        let min=  <T as Bounded>::min_value();
        let max=  <T as Bounded>::max_value();
        Self{
            p_min: Point3::new(max,max,max),
            p_max: Point3::new(min,min,min),
        }
    }

    pub fn from_point(p : &Point3<T>) -> Self{
        Self{
            p_min: p.clone(),
            p_max: p.clone(),
        }
    }

    pub fn from_points(p1 : &Point3<T>, p2 : &Point3<T>) -> Self{
        Self{
            p_min: Point3{
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
                z: p1.z.min(p2.z),
            },
            p_max: Point3 {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
                z: p1.z.max(p2.z),
            },
        }
    }

    pub fn corner(&self, corner : usize) -> Point3<T> {
        Point3{
            x: if (corner & 1) != 0 { self[1].x } else { self[0].x },
            y: if (corner & 2) != 0 { self[1].y } else { self[0].y },
            z: if (corner & 4) != 0 { self[1].z } else { self[0].z },
        }
    }

    pub fn union(&self, other: &Point3<T>) -> Self {
        Bound3{
            p_min: Point3 {
                x: self.p_min.x.min(other.x),
                y: self.p_min.y.min(other.y),
                z: self.p_min.z.min(other.z),
            },
            p_max: Point3 {
                x: self.p_max.x.max(other.x),
                y: self.p_max.y.max(other.y),
                z: self.p_max.z.max(other.z),
            },
        }
    }

    pub fn union_with_box(&self, other : Bound3<T>) -> Self {
        Bound3{
            p_min: Point3 {
                x: self.p_min.x.min(other.p_min.x),
                y: self.p_min.y.min(other.p_min.y),
                z: self.p_min.z.min(other.p_min.z),
            },
            p_max: Point3 {
                x: self.p_max.x.max(other.p_max.x),
                y: self.p_max.y.max(other.p_max.y),
                z: self.p_max.z.max(other.p_max.z),
            },
        }
    }

    pub fn intersection(&self, other : Bound3<T>) -> Self {
        Bound3{
            p_min: Point3 {
                x: self.p_min.x.max(other.p_min.x),
                y: self.p_min.y.max(other.p_min.y),
                z: self.p_min.z.max(other.p_min.z),
            },
            p_max: Point3 {
                x: self.p_max.x.min(other.p_max.x),
                y: self.p_max.y.min(other.p_max.y),
                z: self.p_max.z.min(other.p_max.z),
            },
        }
    }

    pub fn overlaps(&self, other : Bound3<T>) -> bool{
        let x =  self.p_max.x >= other.p_min.x && self.p_min.x <= other.p_max.x;
        let y =  self.p_max.y >= other.p_min.y && self.p_min.y <= other.p_max.y;
        let z =  self.p_max.z >= other.p_min.z && self.p_min.z <= other.p_max.z;

        x && y && z
    }

    pub fn inside(&self, p : Point3<T>) -> bool {
            p.x >= self.p_min.x && p.x <= self.p_max.x
        &&  p.y >= self.p_min.y && p.y <= self.p_max.y
        &&  p.z >= self.p_min.z && p.z <= self.p_max.z
    }

    /// A variant of `inside()` that doesn't consider points
    /// In the upper boundary to be inside the boundary
    pub fn inside_exclusive(&self, p : Point3<T>) -> bool {
                p.x >= self.p_min.x && p.x < self.p_max.x
            &&  p.y >= self.p_min.y && p.y < self.p_max.y
            &&  p.z >= self.p_min.z && p.z < self.p_max.z
    }

    pub fn expands(&self, delta : T) -> Self {
        Bound3{
            p_min: self.p_min - Vector3::new(delta, delta, delta),
            p_max: self.p_max + Vector3::new(delta, delta, delta),
        }
    }

    /// Return a Vector along the box diagonal from the min point to max point
    pub fn diagonal(&self) -> Vector3<T>{
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> T{
        let d = self.diagonal();
        T::from_u8(2).unwrap() * (d.x * d.x + d.y * d.y + d.z * d.z)
    }

    pub fn volume(&self) -> T{
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn max_extent(&self) -> i32 {
        let d = self.diagonal();

        if (d.x > d.y && d.x > d.z){
            0
        }else if (d.y > d.z){
            1
        }else {
            2
        }
    }

    /// Compute the linear interpolation 
    /// between the corners of the box by the given amount in each dimension.
    pub fn lerp(&self, p : &Point3<T>) -> Point3<T> {
        Point3{
            x: lerp(p.x, self.p_min.x, self.p_max.x),
            y: lerp(p.y, self.p_min.y, self.p_max.y),
            z: lerp(p.z, self.p_min.z, self.p_max.z),
        }
    }

    pub fn offset(&self, p : Point3<T>) -> Vector3<T>{
        let mut o = p - self.p_min;
        if (self.p_max.x > self.p_min.x){
            o.x = o.x / (self.p_max.x - self.p_min.x);
        }
        if (self.p_max.y > self.p_min.y){
            o.y = o.y / (self.p_max.y - self.p_min.y);
        }
        if (self.p_max.z > self.p_min.z){
            o.z = o.z / (self.p_max.z - self.p_min.z);
        }

        o
    }

    /// Return the Origin Point and the radius of a sphere that bounds the bounding box
    pub fn bounding_sphere(&self) -> (Point3<T>, T){
        let center =  (self.p_min + self.p_max) * T::from_f32(0.5).unwrap() ;
        let radius = if (self.inside(center)){
            self.p_max.distance(center)
        }else { T::from_f32(0.0).unwrap() };

        (center, radius)
    }
}

impl<T> Index<usize> for Bound3<T>{
    type Output = Point3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("Index out of bounds for Vector3: {}", index),
        }
    }
}

type Bound2f = Bound2<f32>;
type Bound3f = Bound3<f32>;
type Bound2i = Bound2<i32>;
type Bound3i = Bound3<i32>;