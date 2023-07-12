use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use std::{
    fs::File,
    process::exit,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use camera::*;
use hittable::*;
use scene::*;
use utility::*;

mod aabb;
mod aarect;
mod camera;
mod cornell_box;
mod hittable;
mod hittable_list;
mod material;
mod onb;
mod ray;
mod scene;
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
        let mut pdf = 0.;
        let mat = rec.mat_ptr.clone();
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if !mat.scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf) {
            return emitted;
        }
        let mut to_light = Vec3::new(randrange(213., 343.), 554., randrange(227., 332.)) - rec.p;
        let distance_squared = to_light.length_squared();
        to_light = to_light.unit();
        if dot(to_light, rec.normal) < 0. {
            return emitted;
        }
        let light_area = (343. - 213.) * (332. - 227.);
        let light_cosine = to_light.y.abs();
        if light_cosine < 0.000001 {
            return emitted;
        }
        pdf = distance_squared / (light_cosine * light_area);
        scattered = Ray::new(rec.p, to_light, r.time());
        emitted
            + attenuation
                * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                * ray_color(&scattered, background, world, depth - 1)
                / pdf
    } else {
        background
    }
}

fn main() {
    let path = std::path::Path::new("output/book3/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let aspect_ratio: f64 = 1.0;
    let width: u32 = 600;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50; //TODO
    let time0 = 0.;
    let time1 = 1.;
    let quality: u8 = 100;

    // World & Camera
    let lookfrom = Point3::new(278., 278., -800.);
    let lookat = Point3::new(278., 278., 0.);
    let vfov = 40.;
    let aperture = 0.;

    let world = cornell_box();
    let background = Color::new(0., 0., 0.);

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
    let multi_progress = MultiProgress::new();

    // Render
    const THREAD_NUM: usize = 14;
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let mut task_list: Vec<Vec<(u32, u32)>> = vec![Vec::new(); THREAD_NUM];
    let mut receiver_list = Vec::new();
    let mut k = 0;
    for j in 0..height {
        for i in 0..width {
            task_list[k].push((i, j));
            k = (k + 1) % THREAD_NUM;
        }
    }

    for task in task_list {
        let (tx, rx) = mpsc::channel();
        receiver_list.push(rx);
        let world_ = world.clone();
        let progress_bar = multi_progress.add(ProgressBar::new(
            (width * height / THREAD_NUM as u32) as u64,
        ));
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            for (i, j) in task {
                let mut pixel_color = Color::default();
                for _s in 0..samples_per_pixel {
                    let u = ((i as f64) + random()) / ((width - 1) as f64);
                    let v = ((j as f64) + random()) / ((height - 1) as f64);
                    let ray = cam.get_ray(u, v, time0, time1);
                    pixel_color += ray_color(&ray, background, &world_, max_depth);
                }
                pixel_color /= samples_per_pixel as f64;
                for _i in 0..3 {
                    *pixel_color.at(_i) = clamp(pixel_color.get(_i).sqrt(), 0., 0.99);
                }
                pixel_color *= 256.;
                result.push((i, j, pixel_color));
                progress_bar.inc(1);
            }
            tx.send(result).unwrap();
            progress_bar.finish();
        });
        threads.push(handle);
    }
    multi_progress.join_and_clear().unwrap();

    for receiver in receiver_list {
        let result = receiver.recv().unwrap();
        for (i, j, pixel_color) in result {
            let (r, g, b) = (pixel_color.x, pixel_color.y, pixel_color.z);
            let pixel = img.get_pixel_mut(i, height - 1 - j);
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }

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
