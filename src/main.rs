extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::scene::Object;

use std::path::Path;
use kiss3d::scene::SceneNode;

use na::{Vector3, Point3, Isometry3, UnitQuaternion, Translation2, Translation3};

fn main() {

    let mut win = Window::new("model");
    win.set_light(Light::StickToCamera);

    let path1 = Path::new("objects/object1.obj");
    let path2 = Path::new("objects/object2.obj");
    let path3 = Path::new("objects/object3.obj");
    let path4 = Path::new("objects/object4.obj");
    let path5 = Path::new("objects/object5.obj");

    let dir = Path::new("objects/");

    let eye = Point3::new(75.0, 100.0, 75.0);
    let at = Point3::new(0.0, 40.0, 0.0);
    let mut arc_ball = ArcBall::new(eye, at);

    let mut obj1 = win.add_obj(path1, dir, standard_scale());
    let mut obj2 = win.add_obj(path2, dir, standard_scale());
    let mut obj3 = win.add_obj(path3, dir, standard_scale());
    //let mut obj4 = win.add_obj(path4, dir, standard_scale());
    //let mut obj5 = win.add_obj(path5, dir, standard_scale());

    let mut top = win.add_group();
    top.add_obj(path4, dir, standard_scale());
    top.add_obj(path5, dir, standard_scale());

    obj1.set_color(1.0, 1.0, 0.0);
    obj2.set_color(1.0, 0.0, 1.0);
    obj3.set_color(0.0, 1.0, 1.0);
    top.set_color(1.0, 1.0, 1.0);

    let mut vec = vec![&mut top, &mut obj3];
    let mut top_group = GroupToRotate::from(vec);
    top_group.rotate(5.0);

    while win.render_with_camera(&mut arc_ball){

        let translation = Vector3::new(0.0, 0.0, 0.0);
        let axiangle = Vector3::new(0.0, 0.0, 0.014);
        let transformation = Isometry3::new(translation, axiangle);

        let unit = UnitQuaternion::new(Vector3::new(0.0, 0.0, 0.01));

        win.draw_point(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0));

    }
}

fn standard_scale() -> Vector3<f32> {

    let scale = Vector3::new(0.1, 0.1, 0.1);
    scale

}

struct GroupToRotate<'a> {
    axis_type: &'a str,
    objects: Vec<&'a mut SceneNode>,
}

impl<'a> GroupToRotate<'_> {
    fn from(objects: Vec<&'a mut SceneNode>) -> GroupToRotate {
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

    fn rotate(&mut self, corner: f32) {
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
