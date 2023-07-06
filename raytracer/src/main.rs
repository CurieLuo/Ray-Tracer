use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

use camera::*;
use hittable::*;
// use hittable_list::*;
// use material::*;
use scene::*;
// use sphere::*;
use utility::*;

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod scene;
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
    let path = std::path::Path::new("output/book1/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let aspect_ratio: f64 = 3. / 2.;
    let width: u32 = 1200;
    let height: u32 = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: i32 = 500;
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
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
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
