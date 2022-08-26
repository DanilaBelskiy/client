extern crate kiss3d;
extern crate nalgebra as na;

use std::fmt::Formatter;
use client_server::{standard_scale};
use client_server::GroupToRotate;

pub mod corner_memory;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::scene::Object;
use kiss3d::scene::SceneNode;

use std::path::Path;
use std::time::{Duration, Instant};

use na::{Vector3, Point3, Isometry3, UnitQuaternion, Translation2, Translation3};
use crate::corner_memory::CornerMemory;

fn main() {

    //Creating window-------------------------------------------------------------------------------

    let mut win = Window::new("model");
    win.set_light(Light::StickToCamera);

    //Creating paths for objects--------------------------------------------------------------------

    let path1 = Path::new("objects/object1.obj");
    let path2 = Path::new("objects/object2.obj");
    let path3 = Path::new("objects/object3.obj");
    let path4 = Path::new("objects/object4.obj");
    let path5 = Path::new("objects/object5.obj");

    let dir = Path::new("objects/");

    //Creating camera-------------------------------------------------------------------------------

    let eye = Point3::new(75.0, 100.0, 75.0);
    let at = Point3::new(0.0, 40.0, 0.0);
    let mut arc_ball = ArcBall::new(eye, at);

    //Creating objects------------------------------------------------------------------------------

    let mut obj1 = win.add_obj(path1, dir, standard_scale());
    let mut obj2 = win.add_obj(path2, dir, standard_scale());
    let mut obj3 = win.add_obj(path3, dir, standard_scale());
    //let mut obj4 = win.add_obj(path4, dir, standard_scale());
    //let mut obj5 = win.add_obj(path5, dir, standard_scale());

    let mut top = win.add_group();
    top.add_obj(path4, dir, standard_scale());
    top.add_obj(path5, dir, standard_scale());

    //Coloring--------------------------------------------------------------------------------------

    //obj1.set_color(1.0, 1.0, 0.0);
    //obj2.set_color(1.0, 0.0, 1.0);
    //obj3.set_color(0.0, 1.0, 1.0);
    //top.set_color(1.0, 1.0, 1.0);

    //Creating CornerMemory object------------------------------------------------------------------

    let mut mem = client_server::corner_memory::CornerMemory::new();

    //Rendering-------------------------------------------------------------------------------------

    let start_time = Instant::now();
    let mut in_start = false;
    let mut in_start1 = false;
    while win.render_with_camera(&mut arc_ball){

        let now = start_time.elapsed();
        win.draw_point(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 1.0, 1.0));

        let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
        let mut group = GroupToRotate::from(vec, "D", &mut mem);
        group.rotate(0.03);

        let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
        let mut group = GroupToRotate::from(vec, "B", &mut mem);
        group.rotate(0.03);

        let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
        let mut group = GroupToRotate::from(vec, "A", &mut mem);
        group.rotate(0.03);

        let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
        let mut group = GroupToRotate::from(vec, "C", &mut mem);
        group.rotate(0.03);

    }
}


