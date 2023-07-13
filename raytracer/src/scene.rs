use crate::{
    aarect::*, bvh::*, cornell_box::*, hittable::*, hittable_list::*, material::*, medium::*,
    sphere::*, texture::*, utility::*,
};

pub fn final_scene() -> (HittableList, Arc<dyn Hittable>) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
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
            boxes1.add(Arc::new(CornellBox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();
    objects.add(Arc::new(BvhNode::new(&boxes1, 0., 1.)));

    let light = Arc::new(DiffuseLight::new_color(Color::new(7., 7., 7.)));
    let light1 = Arc::new(XZRect::new(123., 423., 147., 412., 554., light));
    objects.add(Arc::new(FlipFace::new(light1.clone())));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.,
        1.,
        50.,
        moving_sphere_material,
    ))); //TODO moving_sphere pdf related methods

    objects.add(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let emat = Arc::new(Lambertian::new_texture(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));

    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;

    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::randrange(0., 165.),
            10.,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new(&boxes2, 0., 1.)), 15.)),
        Vec3::new(-100., 270., 395.),
    )));

    (objects, light1)
}

pub fn cornell_box() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15., 15., 15.)));

    let light1 = Arc::new(XZRect::new(213., 343., 227., 332., 554., light));
    lights.add(light1.clone());
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
    objects.add(Arc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    // let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.));
    let box1 = Arc::new(CornellBox::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white,
    ));
    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(box1, 15.)),
        Vec3::new(265., 0., 295.),
    ));
    objects.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    let ball1 = Arc::new(Sphere::new(Point3::new(190., 90., 90.), 90., glass));
    lights.add(ball1.clone());
    objects.add(ball1);

    // let box2 = Arc::new(CornellBox::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 165., 165.),
    //     white,
    // ));
    // let box2 = Arc::new(Translate::new(
    //     Arc::new(RotateY::new(box2, -18.)),
    //     Vec3::new(130., 0., 65.),
    // ));
    // objects.add(box2);

    (objects, Arc::new(lights))
}

pub fn simple_light() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(pertext.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4., 4., 4.)));
    let light1 = Arc::new(Sphere::new(Point3::new(0., 7., 0.), 2., difflight.clone()));
    lights.add(light1.clone());
    objects.add(light1);
    let light2 = Arc::new(XYRect::new(3., 5., 1., 3., -2., difflight));
    lights.add(light2.clone());
    objects.add(light2);

    (objects, Arc::new(lights))
}
