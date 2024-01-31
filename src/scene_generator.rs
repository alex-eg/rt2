use crate::animation::Animation;
use crate::camera::CamBuilder;
use crate::light::Light;
use crate::material::Material;
use crate::math::Vec3f;
use crate::object::{new_box, new_sphere, new_square, new_triangle, Object};
use crate::scene::Scene;

use ron::ser::{to_string_pretty, PrettyConfig};

#[allow(dead_code)]
pub fn generate_scene() {
    let red = Material {
        ambient: Vec3f::new(0.1, 0.1, 0.1),
        diffuse: Vec3f::new(1., 0., 0.),
        specular: Vec3f::new(1., 1., 1.),
        shininess: 30.0,
        reflection: 0.8,
        refraction: 0.,
    };

    let blue = Material {
        ambient: Vec3f::new(0.1, 0.1, 0.1),
        diffuse: Vec3f::new(0., 0.3, 1.),
        specular: Vec3f::new(1., 1., 1.),
        shininess: 10.0,
        reflection: 0.4,
        refraction: 1.025,
    };

    let green = Material {
        ambient: Vec3f::new(0.1, 0.1, 0.1),
        diffuse: Vec3f::new(0., 1., 0.3),
        specular: Vec3f::new(1., 1., 1.),
        shininess: 10.0,
        reflection: 0.8,
        refraction: 0.,
    };

    let camera = CamBuilder::new()
        .eye(Vec3f::new(0., 0., 60.))
        .center(Vec3f::new(0., 0., 59.))
        .fov(30.)
        .width(128)
        .height(128)
        .up(Vec3f::new(0., -1., 0.))
        .build();

    let mut sphere1 = new_sphere("s1", Vec3f::new(15., 15., 15.), 5., green);
    let mut box1 = new_box(
        "b1",
        Vec3f::new(10., 10., -20.),
        Vec3f::new(10., 10., 10.),
        red,
    );
    let mut sphere3 = new_sphere("s5", Vec3f::new(-15., 15., 15.), 5., blue);
    let mut sphere4 = new_sphere("s6", Vec3f::new(-15., 15., -15.), 5., red);
    let sphere5 = new_sphere("s3", Vec3f::new(15., -15., 15.), 5., green);
    let sphere6 = new_sphere("s4", Vec3f::new(15., -15., -15.), 5., blue);
    let sphere7 = new_sphere("s7", Vec3f::new(-15., -15., 15.), 5., red);
    let sphere8 = new_sphere("s8", Vec3f::new(-15., -15., -15.), 5., green);
    sphere1.set_animation(Animation::new(&sphere1, &[Vec3f::new(0., -10., 0.)]));
    box1.set_animation(Animation::new(&box1, &[Vec3f::new(0., 7., 0.)]));
    sphere3.set_animation(Animation::new(&sphere3, &[Vec3f::new(0., -8., 0.)]));
    sphere4.set_animation(Animation::new(&sphere4, &[Vec3f::new(0., 5., 0.)]));

    let triangle = new_triangle(
        "tri1",
        Vec3f::new(0., -10., -10.),
        Vec3f::new(-10., 10., 0.),
        Vec3f::new(10., 10., 0.),
        blue,
    );

    let objects: Vec<Object> = vec![
        sphere1,
        box1,
        sphere3,
        sphere4,
        sphere5,
        sphere6,
        sphere7,
        triangle,
        sphere8,
        new_square("square", Vec3f::new(0., 30., 0.), 128, blue),
    ];

    let light1 = Light {
        pos: Vec3f::new(0., -60., 0.),
        color: Vec3f::new(1., 1., 1.),
    };
    let lights: Vec<Light> = vec![light1];

    let scene: Scene = Scene{ cam: camera, objects: objects, lights: lights };
    let s = to_string_pretty(&scene, PrettyConfig::default()).unwrap();
    println!("{}", s);
}
