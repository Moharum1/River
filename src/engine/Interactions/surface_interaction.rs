use crate::engine::Interactions::{Interactions, MediumInterface};
use crate::engine::math::Normal::{Normal3, Normal3f};
use crate::engine::math::Point::{Point2f, Point3f};
use crate::engine::math::Vector::Vector3f;
use crate::engine::primitives::Shape;

struct Shading{
    normal: Normal3f,
    dp_du : Vector3f,
    dp_dv : Vector3f,
    dn_du : Normal3f,
    dn_dv : Normal3f,
}

struct SurfaceInteraction{
    point: Point3f,
    normal: Normal3f,
    point_error: Vector3f,
    wo: Vector3f,
    medium_interface : Box<dyn MediumInterface>, //TODO:FIX

    uv : Point2f,
    dp_du : Vector3f,
    dp_dv : Vector3f,
    dn_du : Normal3f,
    dn_dv : Normal3f,
    //shape : Option<Shape> //TODO:FIX
}

impl Interactions for SurfaceInteraction {
    fn new(point: Point3f, normal: Normal3f, point_error: Vector3f, wo: Vector3f, medium_interface: Box<dyn MediumInterface>) -> Self {
        Self{
            point,
            normal,
            point_error,
            wo,
            medium_interface,
            uv: Default::default(),
            dp_du: Default::default(),
            dp_dv: Default::default(),
            dn_du: Default::default(),
            dn_dv: Default::default(),
        }
    }
}

impl SurfaceInteraction {
    fn new_surface(
        point: Point3f, normal: Normal3f, point_error: Vector3f, wo: Vector3f, medium_interface: Box<dyn MediumInterface>,
        uv : Point2f, dp_du : Vector3f, dp_dv : Vector3f, dn_du : Normal3f, dn_dv : Normal3f, shape : Option<impl Shape>
    ) -> Self{
        let mut surface = Self{
            point,
            normal,
            point_error,
            wo,
            medium_interface,
            uv,
            dp_du,
            dp_dv,
            dn_du,
            dn_dv,
            //shape,
        };

        let mut shader = Shading{
            normal,
            dp_du,
            dp_dv,
            dn_du,
            dn_dv,
        };

        //TODO
        // if (shape.is_some() && shape.unwrap().reverseOrientation() ^ shape.unwrap().transformSwapsHandedness()){
        //     surface.normal = -1;
        //     shader.normal = -1;
        // }

        surface
    }

    fn set_shading_geometry(
        &mut self, shader: &mut Shading, dp_du : Vector3f, dp_dv : Vector3f, dn_du : Normal3f, dn_dv : Normal3f, orientation_is_authorative : bool){
        let data = dp_du.cross(&dp_dv)
            .normalize();
        shader.normal = Normal3f{
            x: data.x,
            y: data.y,
            z: data.z,
        };

        //TODO
        // if (shape.is_some() && shape.unwrap().reverseOrientation() ^ shape.unwrap().transformSwapsHandedness()){
        //     surface.normal = -1;
        //     shader.normal = -1;
        // }

        if (orientation_is_authorative){
            self.normal = shader.normal.face_forward(&self.normal);
        }else {
            shader.normal = shader.normal.face_forward(&self.normal);
        }
    }

    //TODO
    //Transform()
}