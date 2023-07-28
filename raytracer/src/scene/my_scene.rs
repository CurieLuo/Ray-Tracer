use crate::{
    hittable::{bvh::*, sphere::Sphere, *},
    material::*,
    obj_loader::*,
    texture::*,
    utility::*,
};
use ndarray::Array2;

pub fn scifi1() -> (HittableList, HittableList) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    let light = Box::new(Sphere::new(
        &Point3::new(-10., 5., 15.),
        5.,
        DiffuseLight::new_from_color(&Color::grayscale(4.)),
    ));
    // let light = Box::new(Sphere::new(
    //     Point3::new(-20., 10., 30.),
    //     2.,
    //     DiffuseLight::new_color(Color::new(1., 1., 1.) * 150.),
    // ));
    lights.add(light.clone());
    world.add(light);

    let mut ground = HittableList::new();
    let scale = 4.;
    for i in -4..=4 {
        for j in -4..=0 {
            ground.add(object(
                "Sci-Fi-Floor",
                scale,
                0.,
                0.,
                0.,
                [i as f64 * scale, -3., j as f64 * scale],
            ));
        }
    }
    world.add(Box::new(BVHNode::new(ground, 0., 1.)));

    world.add(object("sjtu", 1.65, 0., -20., 0., [4., -2.4, 0.5]));

    let mut droid_and_sword = HittableList::new();
    droid_and_sword.add(object(
        "StingSword",
        1. / 30.,
        90.,
        -150.,
        0.,
        [1.5 - 0.1, -1.5, 0.2],
    ));
    droid_and_sword.add(object("droid", 0.8, 0., 0., 0., [0., -2.8, 0.]));
    world.add(Box::new(RotateY::new(droid_and_sword, -15.)));

    match 0 {
        0 => world.add(object(
            "SciFi_Fighter",
            1. / 200.,
            20.,
            30.,
            0.,
            [-10., 6., -15.],
        )),
        _ => world.add(object(
            "space_battleship_lp",
            1. / 2.,
            27.,
            60.,
            0.,
            [-10., 4.2, -15.],
        )),
    };

    world.add(object("UFO", 1.5, 18., 0., -10., [7., 4., -12.]));
    world.add(object("Plasma_turret", 1., 0., 30., 0., [6., -2.4, -8.]));

    world.add(object(
        "CartoonSpaceRocket",
        0.7,
        0.,
        0.,
        0.,
        [-6., -2.8, -8.],
    ));

    world.add(object("astronaut", 0.7, 0., 70., 0., [-7.5, -2.8, -6.]));
    world.add(object("TimeBomb", 0.4, 0., 10., -120., [-5., -2., -2.]));

    world.add(Box::new(Sphere::new(
        &Point3::new(0., 11., -30.),
        2.2,
        Lambertian::new(ImageTexture::new("image/colorful3.jpg")),
    )));

    (world, lights)
}

pub fn object(
    name: &str,
    scale: f64,
    rotx: f64,
    roty: f64,
    rotz: f64,
    pos: [f64; 3],
) -> Box<dyn Hittable> {
    Box::new(load_obj_and_mtl(
        format!("object/{}/", name).as_str(),
        (name.to_owned() + ".obj").as_str(),
        scale,
        rot_x(rotx).dot(&rot_y(roty)).dot(&rot_z(rotz)),
        Vec3::from_array(&pos),
    ))
}

pub fn rot_x(angle: f64) -> Array2<f64> {
    let theta = angle.to_radians();
    let (cosine, sine) = (theta.cos(), theta.sin());
    let mut mat = Array2::<f64>::eye(3);
    mat[(1, 1)] = cosine;
    mat[(2, 2)] = cosine;
    mat[(1, 2)] = -sine;
    mat[(2, 1)] = sine;
    mat
}
pub fn rot_y(angle: f64) -> Array2<f64> {
    let theta = angle.to_radians();
    let (cosine, sine) = (theta.cos(), theta.sin());
    let mut mat = Array2::<f64>::eye(3);
    mat[(2, 2)] = cosine;
    mat[(0, 0)] = cosine;
    mat[(2, 0)] = -sine;
    mat[(0, 2)] = sine;
    mat
}
pub fn rot_z(angle: f64) -> Array2<f64> {
    let theta = angle.to_radians();
    let (cosine, sine) = (theta.cos(), theta.sin());
    let mut mat = Array2::<f64>::eye(3);
    mat[(0, 0)] = cosine;
    mat[(1, 1)] = cosine;
    mat[(0, 1)] = -sine;
    mat[(1, 0)] = sine;
    mat
}
