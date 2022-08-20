extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{ArcBall, Camera};

use std::path::Path;

use na::{Vector3, Point3};

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

    let mut obj1 = win.add_obj(path1, dir, Vector3::new(0.1, 0.1, 0.1));
    let mut obj2 = win.add_obj(path2, dir, Vector3::new(0.1, 0.1, 0.1));
    let mut obj3 = win.add_obj(path3, dir, Vector3::new(0.1, 0.1, 0.1));
    let mut obj4 = win.add_obj(path4, dir, Vector3::new(0.1, 0.1, 0.1));
    let mut obj5 = win.add_obj(path5, dir, Vector3::new(0.1, 0.1, 0.1));

    let mut status = String::from("Too close");

    while win.render_with_camera(&mut arc_ball){

        let distance = arc_ball.dist();
        if status == "Too close"{
            arc_ball.set_dist(distance+3.0);
        } else if status == "Too far" {
            arc_ball.set_dist(distance-3.0);
        }

        if distance > 500.0 {
            status = String::from("Too far");
        } else if distance < 100.0 {
            status = String::from("Too close");
        }

    }
}