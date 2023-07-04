use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
use std::{fs::File, /*, mem::Discriminant*/ process::exit};
use vec3::Vec3;

mod ray;
mod vec3;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().squared_len();
    let b = oc * r.direction() * 2.;
    let c = oc.squared_len() - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant > 0.
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0., 0., -1.), 0.5, r) {
        return Vec3::new(1., 0., 0.);
    }
    let t = 0.5 * (r.direction().make_unit_vector().y + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn main() {
    let path = std::path::Path::new("output/book1/image3.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let width = 256;
    let height = 256;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let lower_left_corner = Vec3::new(-1., -1., -1.);
    let horizontal = Vec3::new(2., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);

            let u = (i as f64) / (width as f64);
            let v = (j as f64) / (height as f64);
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(ray) * 255.99;
            let (r, g, b) = (col.get(0), col.get(1), col.get(2));
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
