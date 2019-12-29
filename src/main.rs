mod camera;
mod material;
mod model;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use material::{Material, Scatter};
use model::Model;
use ray::Ray;
use vec3::{vec3, Vec3};

use image::{ImageBuffer, Rgb, RgbImage};

use indicatif::{ProgressBar, ProgressStyle};

use rayon::prelude::*;

use std::io;

fn main() -> io::Result<()> {
    let mat_1 = Material::lambertian(vec3(0.1, 0.2, 0.5));
    let mat_2 = Material::lambertian(vec3(0.8, 0.8, 0.0));
    let mat_3 = Material::metal(vec3(0.8, 0.6, 0.2), 0.0);
    let mat_4 = Material::dielectric(1.5);
    let mat_5 = Material::diffuse_light(vec3(1.0, 0.9, 0.4));
    let world = Model::list(vec![
        Model::sphere(vec3(0.0, -0.3, -1.0), 0.2, &mat_1),
        Model::sphere(vec3(0.0, -100.5, -1.0), 100.0, &mat_2),
        Model::sphere(vec3(1.0, 0.0, -1.0), 0.5, &mat_3),
        Model::sphere(vec3(0.0, 0.0, -2.0), 0.5, &mat_4),
        Model::sphere(vec3(-1.0, 0.0, -1.0), 0.40, &mat_5),
        Model::sphere(vec3(-1.0, 5.0, -1.0), 0.40, &mat_5),
    ]);

    let nx = 900u32;
    let ny = 600u32;
    let ns = 500u32;

    let total_size = nx * ny;

    let pb = ProgressBar::new(total_size.into());
    pb.set_style(ProgressStyle::default_bar()
        .template("Rendering {spinner:.green} [{elapsed_precise}] {percent:>3}% [{bar:40.cyan/blue}] {pos}/{len} pixels ({per_sec} | {eta})")
        .progress_chars("#>-"));

    let look_from = vec3(-3.0, 3.0, 2.0);
    let look_at = vec3(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vec3(0.0, 1.0, 0.0),
        20.0,
        f64::from(nx) / f64::from(ny),
        aperture,
        dist_to_focus,
    );

    let vec = (0..ny)
        .into_par_iter()
        .flat_map(|j| {
            (0..nx)
                .into_par_iter()
                .map(|i| {
                    let mut col = (0..ns)
                        .into_par_iter()
                        .map(|_| {
                            (
                                (f64::from(i) + rand::random::<f64>()) / f64::from(nx),
                                (f64::from(j) + rand::random::<f64>()) / f64::from(ny),
                            )
                        })
                        .map(|(u, v)| camera.get_ray(u, v))
                        .map(|ray| color(ray, &world))
                        .reduce(|| Vec3::ZERO, |a, b| a + b);
                    pb.inc(1);
                    col = 255.99 * (col / f64::from(ns)).sqrt().coerce();
                    (i, j, Rgb([col.x as u8, col.y as u8, col.z as u8]))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Rendering complete!");

    let mut buf: RgbImage = ImageBuffer::new(nx, ny);

    println!("Writing to file...");

    vec.into_iter()
        .for_each(|(x, y, pixel)| buf.put_pixel(x, y, pixel));

    buf.save("./output/default.png")?;

    println!("Write complete!");

    Ok(())
}

fn color(mut ray: Ray<f64>, world: &Model) -> Vec3<f64> {
    let mut factor = Vec3::ID;
    let mut emit = Vec3::ZERO;
    let depth = 0;

    while let Some(rec) = world.hit(&ray, 0.00001, std::f64::MAX) {
        if depth >= 50 {
            return Vec3::ZERO;
        } else if let Some(Scatter {
            scattered,
            attenuation,
        }) = rec.material.scatter(ray, &rec)
        {
            ray = scattered;
            factor *= attenuation;
            
            emit += rec.material.emit(rec);
        } else {
            return factor * rec.material.emit(rec);
        }
    }

    // let unit_direction = ray.direction.unit();
    // let t = 0.5 * (unit_direction.y + 1.0);
    // let sky_color = (1.0 - t) * Vec3::ID + t * vec3(0.5, 0.7, 1.0);
    let sky_color = Vec3::ZERO;

    factor * (emit + sky_color)
}
