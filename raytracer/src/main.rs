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
use hittable_list::*;
use pdf::*;
use scene::*;
use utility::*;

mod bvh;
mod camera;
mod hittable;
mod material;
mod obj_file;
mod onb;
mod pdf;
mod scene;
mod texture;
mod utility;

fn _ray_color(
    r: &Ray,
    background: Color,
    world: &HittableList,
    lights: &HittableList,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted: Vec3 = rec.mat_ptr.emitted(&rec, rec.u, rec.v, rec.p);
        if let Some(srec) = rec.mat_ptr._scatter(r, &rec) {
            // TODO both specular and diffusive
            if let Some(pdf_ptr) = srec.pdf_ptr {
                if lights._is_empty() {
                    let scattered = Ray::new(rec.p, pdf_ptr.generate().unit(), r.time);
                    let pdf_val = pdf_ptr.value(scattered.direction);

                    emitted
                        + srec.attenuation
                            * rec.mat_ptr._scattering_pdf(r, &rec, &scattered)
                            * _ray_color(&scattered, background, world, lights, depth - 1)
                            / pdf_val
                } else {
                    let light_ptr = HittablePdf::_new(lights, rec.p);
                    let mixed_pdf = MixturePdf::_new(&light_ptr, pdf_ptr.as_ref(), 0.5);
                    let scattered = Ray::new(rec.p, mixed_pdf.generate().unit(), r.time);
                    let pdf_val = mixed_pdf.value(scattered.direction);

                    emitted
                        + srec.attenuation
                            * rec.mat_ptr._scattering_pdf(r, &rec, &scattered)
                            * _ray_color(&scattered, background, world, lights, depth - 1)
                            / pdf_val
                }
            } else {
                emitted
                    + srec.attenuation
                        * _ray_color(&srec.scattered, background, world, lights, depth - 1)
            }
        } else {
            emitted
        }
    } else {
        background
    }
}

fn ray_color(r: &Ray, background: Color, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted: Vec3 = rec.mat_ptr.emitted(&rec, rec.u, rec.v, rec.p);
        if let Some(srec) = rec.mat_ptr.scatter(r, &rec) {
            emitted + srec.attenuation * ray_color(&srec.scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}

fn main() {
    let path = std::path::Path::new("output/test/test2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let mut aspect_ratio: f64 = 1.;
    let width: u32;
    let samples_per_pixel: i32;
    let mut max_depth: i32 = 50;
    let time0 = 0.;
    let time1 = 1.;
    let quality: u8 = 100;

    // World & Camera
    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.;
    let mut background = Color::new(0., 0., 0.);

    let world;
    match 0 {
        0 => {
            world = test();
            aspect_ratio = 16. / 9.;
            width = 600 / 1;
            samples_per_pixel = 100 / 1;
            max_depth = 50 / 1;
            lookfrom = Point3::new(0., 0., 1000.);
            lookat = Point3::new(0., 0., 0.);
            background = Color::new(0.70, 0.80, 1.00);
            vfov = 45.;
        }
        1 => {
            world = random_scene();
            aspect_ratio = 16. / 9.;
            width = 400;
            samples_per_pixel = 100;
            // aspect_ratio = 3. / 2.;
            // width = 1200;
            // samples_per_pixel = 500;
            lookfrom = Point3::new(13., 2., 3.);
            lookat = Point3::new(0., 0., 0.);
            background = Color::new(0.70, 0.80, 1.00);
            aperture = 0.1;
            vfov = 20.;
        }
        2 => {
            world = cornell_box();
            width = 600;
            samples_per_pixel = 200;
            lookfrom = Point3::new(278., 278., -800.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        3 => {
            world = simple_light();
            width = 400;
            samples_per_pixel = 400;
            lookfrom = Point3::new(26., 3., 6.);
            lookat = Point3::new(0., 2., 0.);
            vfov = 20.;
        }
        _ => {
            world = final_scene();
            width = 800;
            samples_per_pixel = 100;
            max_depth = 50;
            lookfrom = Point3::new(478., 278., -600.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
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
    let multi_progress = MultiProgress::new();

    // Render
    const THREAD_NUM: usize = 14;
    const BATCH_SIZE: u32 = 64; // optimize progress bar
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
            (width * height / THREAD_NUM as u32 / BATCH_SIZE) as u64,
        ));
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            let mut progress_count = 0;
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
                    pixel_color[_i] = clamp(pixel_color[_i].sqrt(), 0., 0.99);
                }
                pixel_color *= 256.;
                result.push((i, j, pixel_color));
                progress_count += 1;
                if progress_count % BATCH_SIZE == 0 {
                    progress_bar.inc(1);
                }
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
