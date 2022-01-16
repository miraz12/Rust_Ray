use cgmath::Vector3;
use std::{io::Write, fmt::format};
use rand::prelude::*;

mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod hittablelist;
use hittablelist::{HittableList, Hittable};
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_h = 2.0;
    let viewport_w = aspect_ratio * viewport_h;
    let focal_len = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_w, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_h, 0.0);
    let lower_left_cornet =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_len);

    let mut world = HittableList::default();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let mut out_buffer: String = format!("P3\n{} {}\n255\n", image_width, image_height);

   
    

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = (i as f64 / (image_width as f64 - 1.0)) as f64;
            let v = (j as f64 / (image_height as f64 - 1.0)) as f64;

            let r = Ray::new(
                origin,
                lower_left_cornet + u * horizontal + v * vertical - origin,
            );
            write_color(ray_color_world(r, &world), &mut out_buffer);
        }
    }

    let file = match std::fs::File::create("image.ppm") {
        Err(why) => panic!("Couldn't create file: {}", why),
        Ok(file) => file,
    };
    match write!(&file, "{}", out_buffer) {
        Err(why) => println!("Couldn't write file {}", why),
        _ => (),
    }
    eprintln!("Done.");
}

fn ray_color_world(ray: Ray, world: &HittableList) -> Vector3<f64> {
    match world.hit(&ray, 0.0, 10000.0) {
        Some(r) => 0.5 * (r.normal + Vector3::new(1.0, 1.0, 1.0)),
        None => {
            let len = ray.direction.x * ray.direction.x
                + ray.direction.y * ray.direction.y
                + ray.direction.z * ray.direction.z;
            let unit_vector = ray.direction / len;
            let t = 0.5 * (unit_vector.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn write_color(col: Vector3<f64>, mut out_buffer: &mut String) {
    let ir = (255.999 * col.x) as i32;
    let ig = (255.999 * col.y) as i32;
    let ib = (255.999 * col.z) as i32;

    let color_buff = format!("{} {} {}\n", ir, ig, ib);
    out_buffer.push_str( &color_buff);
}

fn random_double_range(min: f64, max: f64) -> f64{
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}