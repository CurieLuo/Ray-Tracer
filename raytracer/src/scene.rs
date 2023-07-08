use crate::bvh::*;
use crate::{hittable_list::*, material::*, sphere::*, texture::*, utility::*};

pub fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(checker.clone())),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_texture(checker)),
    )));

    world
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
