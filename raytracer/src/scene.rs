use crate::{aarect::*, cornell_box::*, hittable::*, hittable_list::*, material::*, utility::*};

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

    let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.));
    let box1 = Arc::new(CornellBox::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        aluminum,
    ));
    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(box1, 15.)),
        Vec3::new(265., 0., 295.),
    ));
    objects.add(box1);

    let box2 = Arc::new(CornellBox::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Arc::new(Translate::new(
        Arc::new(RotateY::new(box2, -18.)),
        Vec3::new(130., 0., 65.),
    ));
    objects.add(box2);

    (objects, Arc::new(lights))
}
