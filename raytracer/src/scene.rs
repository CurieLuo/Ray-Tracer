use crate::{
    aarect::*, bvh::*, hittable_list::*, material::*, medium::*, obj_file::*, rect_box::*,
    sphere::*, texture::*, transform::*, utility::*,
};

pub fn test() -> (HittableList, HittableList) {
    let mut world;
    let lights;
    (world, lights) = (HittableList::new(), HittableList::new());

    // let albedo = Color::random() * Color::random();
    // let material = Lambertian::new(albedo);

    let ironman_material =
        Lambertian::new_texture(ImageTexture::new("image/Iron_Man/Iron_Man_Diffuse.png"));
    let ironman = Arc::new(Translate::new(
        RotateY::new(
            BvhNode::new(
                &load_obj("object/IronMan.obj", ironman_material, 50.0),
                0.,
                1.,
            ),
            0.,
        ),
        Vec3::new(0., -190., 300.),
    ));
    world.add(ironman);

    let book_material = Lambertian::new_texture(ImageTexture::new("image/book/baseColor.png"));
    let book = Arc::new(Translate::new(
        RotateY::new(
            BvhNode::new(&load_obj("object/book.obj", book_material, 50.0), 0., 1.),
            0.,
        ),
        Vec3::new(0., -220., 300.),
    ));
    world.add(book);

    (world, lights)
}

pub fn final_scene() -> (HittableList, HittableList) {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = randrange(1., 101.);
            let z1 = z0 + w;
            boxes1.add(Arc::new(RectBox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();
    objects.add(Arc::new(BvhNode::new(&boxes1, 0., 1.)));

    let light = DiffuseLight::new_color(Color::new(7., 7., 7.));
    let light1 = XZRect::new(123., 423., 147., 412., 554., light);
    objects.add(Arc::new(FlipFace::new(light1.clone())));
    let mut lights = HittableList::new();
    lights.add(Arc::new(light1));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Lambertian::new(Color::new(0.7, 0.3, 0.1));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.,
        1.,
        50.,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Dielectric::new(1.5),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.),
    )));

    let boundary = Sphere::new(Point3::new(360., 150., 145.), 70., Dielectric::new(1.5));
    objects.add(Arc::new(boundary.clone()));
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Sphere::new(Point3::new(0., 0., 0.), 5000., Dielectric::new(1.5));
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let emat = Lambertian::new_texture(ImageTexture::new("image/earthmap.jpg"));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));

    let pertext = NoiseTexture::new(0.1);
    objects.add(Arc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Lambertian::new_texture(pertext),
    )));
    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;

    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::randrange(0., 165.),
            10.,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        RotateY::new(BvhNode::new(&boxes2, 0., 1.), 15.),
        Vec3::new(-100., 270., 395.),
    )));

    (objects, lights)
}

pub fn cornell_box() -> (HittableList, HittableList) {
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_color(Color::new(15., 15., 15.) * 4.);

    let light1 = XZRect::new(213., 343., 227., 332., 554., light);
    lights.add(Arc::new(light1.clone()));
    objects.add(Arc::new(FlipFace::new(light1)));

    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Arc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(0., 555., 0., 555., 555., white)));

    // let box1 = RectBox::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 330., 165.),
    //     white,
    // );
    // let box1 = Arc::new(Translate::new(
    //     RotateY::new(box1, 15.),
    //     Vec3::new(265., 0., 295.),
    // ));
    // objects.add(box1);

    // let glass = Dielectric::new(1.5);
    // let ball1 = Arc::new(Sphere::new(Point3::new(190., 90., 90.), 90., glass));
    // lights.add(ball1.clone());
    // objects.add(ball1);

    // let box2 = RectBox::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 165., 165.),
    //     white,
    // );
    // let box2 = Arc::new(Translate::new(
    //     RotateY::new(box2, -18.),
    //     Vec3::new(130., 0., 65.),
    // ));
    // objects.add(box2);

    (objects, lights)
}

pub fn simple_light() -> (HittableList, HittableList) {
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let pertext = NoiseTexture::new(4.);
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Lambertian::new_texture(pertext.clone()),
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Lambertian::new_texture(pertext),
    )));

    let difflight = DiffuseLight::new_color(Color::new(4., 4., 4.));
    let light1 = Arc::new(Sphere::new(Point3::new(0., 7., 0.), 2., difflight.clone()));
    lights.add(light1.clone());
    objects.add(light1);
    let light2 = Arc::new(XYRect::new(3., 5., 1., 3., -2., difflight));
    lights.add(light2.clone());
    objects.add(light2);

    (objects, lights)
}

pub fn random_scene() -> (HittableList, HittableList) {
    let time0 = 0.;
    let time1 = 1.;
    let mut world = HittableList::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(
                (a as f64) + 0.9 * random(),
                0.2,
                (b as f64) + 0.9 * random(),
            );
            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0., randrange(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        time0,
                        time1,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::randrange(0.5, 1.);
                    let fuzz = randrange(0., 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material3,
    )));

    let mut world_with_ground = HittableList::new();
    world_with_ground.add(Arc::new(BvhNode::new(&world, time0, time1)));

    //// let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let checker = CheckerTexture::new_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world_with_ground.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Lambertian::new_texture(checker),
    )));

    (world_with_ground, HittableList::new())
}
