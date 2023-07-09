use crate::bvh::*;
use crate::{aarect::*, hittable_list::*, material::*, sphere::*, texture::*, utility::*};

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15., 15., 15.)));

    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XZRect::new(213., 343., 227., 332., 554., light)));
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

    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

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
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        difflight.clone(),
    )));
    objects.add(Arc::new(XYRect::new(3., 5., 1., 3., -2., difflight)));

    objects
}

pub fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    let mut hittable_list = HittableList::new();
    hittable_list.add(globe);
    hittable_list
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
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

    objects
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(checker.clone())),
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(checker)),
    )));

    objects
}

pub fn random_scene() -> HittableList {
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
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
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
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material3,
    )));

    let mut world_with_ground = HittableList::new();
    world_with_ground.add(Arc::new(BvhNode::new(&world, time0, time1)));

    //// let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world_with_ground.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new_texture(checker)),
    )));

    world_with_ground
}
