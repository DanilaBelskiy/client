extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::scene::SceneNode;

use na::{Vector3, UnitQuaternion, Translation3};

pub fn standard_scale() -> Vector3<f32> {

    let scale = Vector3::new(0.1, 0.1, 0.1);
    scale

}

pub struct GroupToRotate<'a> {
    axis_type: &'a str,
    objects: Vec<&'a mut SceneNode>,
}

impl<'a> GroupToRotate<'_> {
    pub fn from(objects: Vec<&'a mut SceneNode>) -> GroupToRotate {
        let mut axis= "";
        if objects.len() == 1 {
            axis = "A";
        } else if objects.len() == 2 {
            axis = "B";
        } else if objects.len() == 3 {
            axis = "C";
        }
        GroupToRotate{
            axis_type: axis,
            objects,
        }
    }

    pub fn rotate(&mut self, corner: f32) {
        match self.axis_type {
            "A" => {
                for i in 0..self.objects.len() {
                    self.objects[i].append_translation(&Translation3::new(0.0, -57.5, 0.0));
                    self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, corner)));
                    self.objects[i].append_translation(&Translation3::new(0.0, 57.5, 0.0));
                }
            }
            "B" => {
                for i in 0..self.objects.len() {
                    self.objects[i].append_translation(&Translation3::new(0.0, -37.5, 0.0));
                    self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, corner)));
                    self.objects[i].append_translation(&Translation3::new(0.0, 37.5, 0.0));
                }
            }
            "C" => {
                for i in 0..self.objects.len() {
                    self.objects[i].append_translation(&Translation3::new(0.0, -15.0, 0.0));
                    self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, corner)));
                    self.objects[i].append_translation(&Translation3::new(0.0, 15.0, 0.0));
                }
            }
            _ => ()
        }
    }
}