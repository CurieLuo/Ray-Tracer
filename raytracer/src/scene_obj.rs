#![allow(dead_code, unused_imports, unused_mut)]
use crate::{
    bvh::*,
    hittable::{
        aarect::{XYRect, XZRect},
        *,
    },
    hittable_list::*,
    material::*,
    obj_file::*,
    scene::*,
    sphere::*,
    texture::*,
    transform::*,
    utility::*,
};
use ndarray::array;

pub fn scene1() -> (HittableList, HittableList) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();
    let sun = Box::new(Sphere::new(
        Vec3::new(-100., 60., -100.),
        65.,
        DiffuseLight::new(ImageTexture::new("image/sun.jpg")),
    ));
    lights.add(sun.clone());
    world.add(sun);
    world.add(Box::new(Sphere::new(
        Vec3::new(50., 30., -50.),
        20.,
        Lambertian::new_texture(ImageTexture::new("image/colorful3.jpg")),
        // Lambertian::new_texture(ImageTexture::new("image/jupitermap.jpg")),
    )));
    let light = Box::new(Sphere::new(
        Vec3::new(-1000., 500., 300.),
        500.,
        DiffuseLight::new_color(Color::new(4., 4., 4.)),
    ));
    lights.add(light.clone());
    world.add(light);
    world.add(object("UFO", 3.5, 40., 0., -40., [-50., 30., -50.]));
    world.add(object("IronMan", 4., 0., -30., 0., [27., -23., 20.]));
    world.add(object(
        "stunt_glyder",
        1. / 15.,
        20.,
        0.,
        0.,
        [-20., -10., 40.],
    ));
    world.add(object(
        "Sirus_Transport",
        1. / 5.5,
        0.,
        -160.,
        0.,
        [0., -30., 0.],
    ));
    (world, lights)
}

pub fn scene2() -> (HittableList, HittableList) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    let light = Box::new(Sphere::new(
        Point3::new(-10., 5., 15.),
        5.,
        DiffuseLight::new_color(Color::new(1., 1., 1.) * 4.),
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
    world.add(Box::new(BvhNode::new(ground, 0., 1.)));

    world.add(object("sjtu", 1.65, 0., -20., 0., [4., -2.4, 0.5]));

    let mut droid_and_sword = HittableList::new();
    droid_and_sword.add(object(
        "StingSword",
        1. / 30.,
        90.,
        -150.,
        0.,
        [1.5 - 0.07, -1.5, 0.2],
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
    world.add(object("TimeBomb", 0.4, 0., 10., -120., [-5., -2.1, -2.]));

    world.add(Box::new(Sphere::new(
        Point3::new(0., 11., -30.),
        2.2,
        Lambertian::new_texture(ImageTexture::new("image/colorful3.jpg")),
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
        ("object/".to_owned() + name + "/").as_str(),
        (name.to_owned() + ".obj").as_str(),
        scale,
        rot_x(rotx).dot(&rot_y(roty)).dot(&rot_z(rotz)),
        Vec3::from_array(&pos),
    ))
}
// pub fn blaster_kit(
//     name: &str,
//     scale: f64,
//     rotx: f64,
//     roty: f64,
//     rotz: f64,
//     pos: [f64; 3],
// ) -> Box<dyn Hittable> {
//     Box::new(load_obj_and_mtl(
//         "object/kenney-blaster-kit/",
//         (name.to_owned() + ".obj").as_str(),
//         scale,
//         rot_x(rotx).dot(&rot_y(roty)).dot(&rot_z(rotz)),
//         Vec3::from_array(&pos),
//     ))
// }

pub fn test1() -> (HittableList, HittableList) {
    let mut world = random_scene();
    // world = HittableList::new();
    // world.add(Box::new(load_obj_and_normal(
    //     "object/cube.obj",
    //     "image/normal_1.jpg",
    //     Metal::new(Color::new(1., 0.7, 0.7), 0.),
    //     1. / 1.,
    // )));
    // world.add(Box::new(load_obj_and_normal(
    //     "object/sol/sol.obj",
    //     "image/normal.jpg",
    //     Lambertian::new(Color::new(0.8, 0.8, 1.)),
    //     1. / 12000.,
    // )));
    // world.add(object("airplane", 1. / 250., -90., 0., 0.,[0., 0., 0.]));
    // world.add(object(
    //     "TransportShuttle",
    //     10. / 20.,
    //     0.,
    //     90.,
    //     0.,
    //     [0., 0., 0.],
    // ));
    // world.add(object("diamonds", 15. / 20., 0., 90., 0., [0., 0., 0.]));
    // world.add(object("Robot", 0.45, 0., 0., 0., [0., 0.8, 0.]));
    // world.add(object("TimeBomb", 0.4, 0., 0., 0., [0., 0., 0.]));
    // world.add(object("UFO", 1.5, 0., 0., 0., [5., -2., -6.]));
    // world.add(object("StylizedPlanets", 2., 0., 0., 0., [15., 10., -30.]));
    // world.add(Box::new(RotateY::new(
    //     load_obj(
    //         "object/cat_lp.obj",
    //         Metal::new(Color::new(1., 0.8, 0.8), 0.),
    //         1. / 150.,
    //     ),
    //     90.,
    // )));
    // world.add(object("rocket_lp", 0.6, 0., -20., 0., [-5.5, -2.9, -8.]));
    // world.add(object(
    //     "StylizedPlanets",
    //     1.8,
    //     0.,
    //     -150.,
    //     0.,
    //     [-2., 10., -30.],
    // ));
    // world.add(object("bee_turret", 1. / 6., 0., -40., 0., [4., 0.4, 0.]));
    (world, HittableList::new())
}
