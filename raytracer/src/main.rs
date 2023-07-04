use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit, sync::Arc};

use hittable::*;
use hittable_list::*;
use ray::*;
use sphere::*;
use vec3::*;

mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: Ray, world: &dyn Hittable) -> Color3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0., f64::INFINITY, &mut rec) {
        return (rec.normal + Color3::new(1., 1., 1.)) * 0.5;
    }
    let t = 0.5 * (r.direction().unit().y + 1.);
    Color3::new(1., 1., 1.) * (1. - t) + Color3::new(0.5, 0.7, 1.) * t
}

fn main() {
    let path = std::path::Path::new("output/book1/image5.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = (width as f64 / aspect_ratio) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    // Progress Bar
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    // World

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
    // Camera
    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;
    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0.0, viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, height - 1 - j);

            let u = (i as f64) / ((width - 1) as f64);
            let v = (j as f64) / ((height - 1) as f64);
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = ray_color(ray, &world) * 255.99;
            let (r, g, b) = (col.get(0), col.get(1), col.get(2));
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
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
