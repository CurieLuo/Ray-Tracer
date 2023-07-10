use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

use camera::*;
use hittable::*;
use scene::*;
use utility::*;

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod cornell_box;
mod hittable;
mod hittable_list;
mod material;
mod medium;
mod perlin;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utility;
mod vec3;

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let mat = rec.mat_ptr.clone();
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}

fn main() {
    let path = std::path::Path::new("output/book2/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let mut aspect_ratio: f64 = 16. / 9.;
    let mut width: u32 = 400;
    let mut samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;
    let time0 = 0.;
    let time1 = 1.;
    let quality: u8 = 100;

    // World & Camera
    let mut lookfrom = Point3::new(13., 2., 3.);
    let mut lookat = Point3::new(0., 0., 0.);
    let mut vfov = 20.;
    let mut aperture = 0.;
    let mut background = Color::new(0.70, 0.80, 1.00);

    let world;
    match 0 {
        1 => {
            world = random_scene();
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
        }
        3 => {
            world = two_perlin_spheres();
        }
        4 => {
            world = earth();
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::new(0., 0., 0.);
            lookfrom = Point3::new(26., 3., 6.);
            lookat = Point3::new(0., 2., 0.);
        }
        _ => {
            world = match 0 {
                1 => cornell_box(),
                _ => cornell_smoke(),
            };
            aspect_ratio = 1.;
            width = 600;
            samples_per_pixel = 200;
            background = Color::new(0., 0., 0.);
            lookfrom = Point3::new(278., 278., -800.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.;
        } // _ => {}
    }

    let height: u32 = (width as f64 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Progress Bar
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    // Render
    for j in 0..height {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, height - 1 - j);
            let mut pixel_color = Color::default();
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + random()) / ((width - 1) as f64);
                let v = ((j as f64) + random()) / ((height - 1) as f64);
                let ray = cam.get_ray(u, v, time0, time1);
                pixel_color += ray_color(&ray, background, &world, max_depth);
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
