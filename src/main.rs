mod camera;
mod material;
mod model;
mod ray;
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
    // Construct the scene.
    let mat_1 = Material::lambertian(vec3(0.1, 0.2, 0.5));
    let mat_2 = Material::lambertian(vec3(0.8, 0.8, 0.0));
    let mat_3 = Material::metal(vec3(0.8, 0.6, 0.2), 0.0);
    let mat_4 = Material::dielectric(1.5);
    let mat_5 = Material::diffuse_light(vec3(1.0, 0.9, 0.4));
    let mat_6 = Material::Combined { scatterer: &mat_3, emitter: &mat_5 };
    let world = Model::list(vec![
        Model::sphere(vec3(0.0, -0.3, -1.0), 0.2, &mat_1),
        Model::sphere(vec3(0.0, -100.5, -1.0), 100.0, &mat_2),
        Model::sphere(vec3(1.0, 0.0, -1.0), 0.5, &mat_3),
        Model::sphere(vec3(0.0, 0.0, -2.0), 0.5, &mat_4),
        Model::sphere(vec3(0.0, 0.0, -2.0), -0.4, &mat_4),
        Model::sphere(vec3(-1.0, 0.0, -1.0), 0.40, &mat_6),
        Model::sphere(vec3(-1.0, 5.0, -1.0), 0.40, &mat_5),
    ]);

    // Image parameters.
    let nx = 900u32;
    let ny = 600u32;
    let ns = 1000u32;

    // Rendering progress bar stuff.
    let total_size = nx * ny;
    let pb = ProgressBar::new(total_size.into());
    pb.set_style(ProgressStyle::default_bar()
        .template("Rendering {spinner:.green} [{elapsed_precise}] {percent:>3}% [{bar:40.cyan/blue}] {pos}/{len} pixels ({per_sec} | {eta})")
        .progress_chars("#>-"));

    // Setting up the camera.
    let look_from = vec3(-3.0, 3.0, 2.0);
    let look_at = vec3(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).mag();
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vec3(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    let mut buf: RgbImage = ImageBuffer::new(nx, ny);

    (0..ny)
        .into_par_iter()
        .flat_map(|j| {
            (0..nx)
                .into_par_iter()
                .map_with(j, |&mut j, i| {
                    let mut col = (0..ns)
                        .into_par_iter()
                        .map(|_| {
                            (
                                (i as f32 + rand::random::<f32>()) / (nx as f32),
                                (j as f32 + rand::random::<f32>()) / (ny as f32),
                            )
                        })
                        .map(|(u, v)| camera.get_ray(u, v))
                        .map(|ray| color(ray, &world, 50))
                        .reduce(|| Vec3::ZERO, |a, b| a + b);
                    col = 255.99
                        * (col / (ns as f32))
                            .map(f32::sqrt)
                            .map(|f| f.max(0.0).min(1.0));
                    (i, j, Rgb([col.x as u8, col.y as u8, col.z as u8]))
                })
                .inspect(|_| pb.inc(1))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(x, y, pixel)| buf.put_pixel(x, y, pixel));

    buf.save("./output/default.png")?;

    Ok(())
}

fn color(mut ray: Ray, world: &Model, max_bounce: i32) -> Vec3 {
    let mut factor = Vec3::ONE;
    let mut emit = Vec3::ZERO;
    let mut bounces = 0;

    while let Some(rec) = world.hit(&ray, 0.00001, std::f32::MAX) {
        // Maximum number of bounces. If exceeded, return the
        // result of all interactions so far with the scene.
        if bounces >= max_bounce {
            break;
        }

        // Get the scattering result from interacting with
        // the material of the object.
        let Scatter {
            scattered,
            attenuation,
        } = rec.material.scatter(ray, &rec);

        // If the ray is completely absorbed, then the only
        // light that could possibly reach the camera is what the
        // material emits.
        if scattered == Ray::ZERO || attenuation == Vec3::ZERO {
            return factor * rec.material.emit(rec);
        }

        ray = scattered;
        factor *= attenuation;
        emit += rec.material.emit(rec);
        bounces += 1;
    }

    // let unit_direction = ray.direction.unit();
    // let t = 0.5 * (unit_direction.y + 1.0);
    // let sky_color = (1.0 - t) * Vec3::ID + t * vec3(0.5, 0.7, 1.0);
    let sky_color = Vec3::ZERO;

    factor * (sky_color + emit)
}