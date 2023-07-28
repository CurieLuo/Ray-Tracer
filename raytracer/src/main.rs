//#![allow(unused)]
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::SliceRandom;
use std::{
    fs::File,
    process::exit,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use camera::*;
use hittable::*;
use material::ScatterRecord;
use my_scene::*;
use pdf::*;
use scene::*;
use texture::*;
use utility::*;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod obj_loader;
pub mod pdf;
pub mod scene;
pub mod texture;
pub mod utility;

const MAX_DEPTH: i32 = 20;

fn ray_color(
    r: &Ray,
    background: &dyn Texture,
    world: &HittableList,
    lights: &HittableList,
    depth: i32,
    (u, v): (f64, f64),
) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, &rec.p);
        let mut srec = ScatterRecord::default();
        if rec.mat_ptr.scatter(r, &rec, &mut srec) {
            if let Some(pdf_ptr) = srec.pdf_ptr {
                if lights.is_empty() {
                    let scattered = Ray::new(&rec.p, &pdf_ptr.generate().unit(), r.time());
                    let pdf_val = pdf_ptr.value(scattered.direction_borrow());

                    emitted
                        + srec.attenuation
                            * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1, (u, v))
                            / pdf_val
                } else {
                    let light_ptr = HittablePDF::new(lights, &rec.p);
                    let mixed_pdf = MixturePDF::new(&light_ptr, pdf_ptr.as_ref());
                    let scattered = Ray::new(&rec.p, &mixed_pdf.generate().unit(), r.time());
                    let pdf_val = mixed_pdf.value(scattered.direction_borrow());

                    emitted
                        + srec.attenuation
                            * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1, (u, v))
                            / pdf_val
                }
            } else {
                emitted
                    + srec.attenuation
                        * ray_color(
                            &srec.scattered,
                            background,
                            world,
                            lights,
                            depth - 1,
                            (u, v),
                        )
            }
        } else {
            emitted
        }
    } else if depth == MAX_DEPTH {
        background.value(u, v, r.origin_borrow())
    } else {
        let dir = r.direction_borrow().unit();
        background.value(0.5 * (dir.x + 1.), 0.5 * (dir.y + 1.), r.origin_borrow())
    }
}

fn main() {
    let path = std::path::Path::new("output/test/test5.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all parent directories");

    // Image
    let mut aspect_ratio: f64 = 1.;
    let width: u32;
    let samples_per_pixel: i32;
    let time0 = 0.;
    let time1 = 1.;
    let quality: u8 = 100;

    // World & Camera
    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.;
    let mut background: Box<dyn Texture> = Box::new(SolidColor::new(&Color::default()));

    let world;
    let mut lights = HittableList::new();
    match 1 {
        1 => {
            (world, lights) = scifi1();
            aspect_ratio = 16. / 9.;
            width = 1600;
            samples_per_pixel = 1000;
            lookfrom = Point3::new(0., 0., 10.);
            lookat = Point3::default();
            // background = Box::new(SolidColor::new(&Color::new(0.70, 0.80, 1.00)));
            background = Box::new(ImageTexture::new("image/milky_way.png"));
            aperture = 0.1;
            vfov = 40.;
        }
        // 2 => {
        //     (world, lights) = test1();
        //     aspect_ratio = 16. / 9.;
        //     width = 600 / 2;
        //     samples_per_pixel = 100 / 2;
        //     lookfrom = Point3::new(13., 2., 3.);
        //     lookat = Point3::default();
        //     background = Box::new(SolidColor::new(&Color::new(0.70, 0.80, 1.00)));
        //     // background = Box::new(ImageTexture::new("image/stars.png"));
        //     aperture = 0.1;
        //     vfov = 20.;
        //     // lookfrom = Point3::new(0., 0., 1000.);
        //     // lookat = Point3::default();
        //     // background = Box::new(SolidColor::new(&Color::new(1.00, 1.00, 1.00)));
        //     // // background = Box::new(ImageTexture::new("image/milky_way.png"));
        //     // vfov = 45.;
        // }
        _ => {
            match 1 {
                1 => {
                    world = random_scene();
                    // aspect_ratio = 16. / 9.;
                    // width = 400;
                    // samples_per_pixel = 100;
                    aspect_ratio = 3. / 2.;
                    width = 1200;
                    samples_per_pixel = 500;
                    lookfrom = Point3::new(13., 2., 3.);
                    lookat = Point3::default();
                    background = Box::new(SolidColor::new(&Color::new(0.70, 0.80, 1.00)));
                    aperture = 0.1;
                    vfov = 20.;
                }
                2 => {
                    (world, lights) = cornell_box();
                    width = 600;
                    samples_per_pixel = 200;
                    lookfrom = Point3::new(278., 278., -800.);
                    lookat = Point3::new(278., 278., 0.);
                    vfov = 40.;
                }
                _ => {
                    (world, lights) = final_scene();
                    width = 800;
                    samples_per_pixel = 100;
                    lookfrom = Point3::new(478., 278., -600.);
                    lookat = Point3::new(278., 278., 0.);
                    vfov = 40.;
                }
            }
        }
    }

    let height: u32 = (width as f64 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    // Progress Bar
    let multi_progress = MultiProgress::new();

    // Render
    const THREAD_NUM: usize = 14;
    const BATCH_SIZE: u32 = 4; // optimize progress bar
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
    for task in task_list.iter_mut() {
        task.shuffle(&mut rand::thread_rng());
    }

    let world = Arc::new(world);
    let lights = Arc::new(lights);
    let background = Arc::from(background);

    for task in task_list {
        let (tx, rx) = mpsc::channel();
        receiver_list.push(rx);
        let world_ = world.clone();
        let lights_ = lights.clone();
        // let background_ = background.clone();
        let background_: Arc<dyn Texture> = Arc::clone(&background);
        let progress_bar = multi_progress.add(ProgressBar::new(
            (width * height / THREAD_NUM as u32 / BATCH_SIZE) as u64,
        ));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(" [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
                .progress_chars("#>-"),
        );
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            let mut progress_count = 0;
            for (i, j) in task {
                let mut pixel_color = Color::default();
                for _s in 0..samples_per_pixel {
                    let u = ((i as f64) + random()) / ((width - 1) as f64);
                    let v = ((j as f64) + random()) / ((height - 1) as f64);
                    let ray = cam.get_ray(u, v, time0, time1);
                    let mut color = ray_color(
                        &ray,
                        background_.as_ref(),
                        world_.as_ref(),
                        lights_.as_ref(),
                        MAX_DEPTH,
                        (u, v),
                    );
                    for _i in 0..3 {
                        if color[_i].is_nan() {
                            color[_i] = 0.;
                        }
                    }
                    // TODO eliminate NaN, not just catch it
                    pixel_color += color;
                    // TODO pdf for generic material
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
            let (r, g, b) = pixel_color.to_tuple();
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
