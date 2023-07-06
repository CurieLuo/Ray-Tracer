use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

use camera::*;
use hittable::*;
use hittable_list::*;
use material::*;
use sphere::*;
use utility::*;

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if let Some(mat) = rec.mat_ptr.clone() {
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(scattered, world, depth - 1);
            }
        }
        return Color::new(0., 0., 0.);
    }
    let t = 0.5 * (r.direction().unit().y + 1.);
    Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.) * t
}

fn main() {
    let path = std::path::Path::new("output/book1/image19.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;
    let quality: u8 = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    // Progress Bar
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.);

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        Arc::new(material_ground),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        Arc::new(material_center),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        Arc::new(material_left),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.45,
        Arc::new(material_left),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        Arc::new(material_right),
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
        20.,
        aspect_ratio,
    );

    // Render
    for j in 0..height {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, height - 1 - j);
            let mut pixel_color = Color::default();
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + random()) / ((width - 1) as f64);
                let v = ((j as f64) + random()) / ((height - 1) as f64);
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, max_depth);
            }
            pixel_color /= samples_per_pixel as f64;
            for _i in 0..3 {
                *pixel_color.at(_i) = clamp(pixel_color.get(_i).sqrt(), 0., 0.99);
            }
            pixel_color *= 256.;
            let (r, g, b) = (pixel_color.get(0), pixel_color.get(1), pixel_color.get(2));
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
            progress.inc(1);
        }
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Failed to output image").red()),
    }

    exit(0);
}
