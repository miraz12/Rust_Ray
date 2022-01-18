use cgmath::{Vector3, InnerSpace};
use std::io::Write;
use rand::prelude::*;

mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod hittablelist;
use hittablelist::{HittableList, Hittable};
mod camera;
use camera::Camera;

use crate::material::{Lambertian, Metal};
mod material;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let cam = Camera::new();

    let mut world = HittableList::default();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Lambertian{albedo: Vector3::new(0.7, 0.3, 0.3)}));
    world.add(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Lambertian{albedo: Vector3::new(0.8, 0.8, 0.0)}));
    world.add(Sphere::new(Vector3::new(-1.0,    0.0, -1.0), 0.5, Metal{albedo: Vector3::new(0.8, 0.8, 0.8)}));

    let mut out_buffer: String = format!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _k in 0..samples_per_pixel {
                let u = ((i as f64 + random_double_range(0.0, 1.0)) / (image_width as f64 - 1.0)) as f64;
                let v = ((j as f64 + random_double_range(0.0, 1.0) )/ (image_height as f64 - 1.0)) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color_world(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel, &mut out_buffer);
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

fn ray_color_world(ray: Ray, world: &HittableList, depth: i32) -> Vector3<f64> {
    if depth <= 0 { // Don't keep bouncing the ray if max detph is reached.
        return Vector3::new(0.0, 0.0, 0.0);
    }
    match world.hit(&ray, 0.001, f64::MAX) { // Shoot ray into scene.
        Some(rec) => { // Bounce ray in world.
            match rec.material.scatter(&ray, &rec) {
                Some(ret) => {
                    let bounce = ray_color_world(ret.0, world, depth-1);
                    return Vector3::new(ret.1.x*bounce.x, ret.1.y*bounce.y, ret.1.z*bounce.z);
                }
                None => {
                    return Vector3::new(0.0, 0.0, 0.0);
                }
            }
        },
        None => { // Nothing hit, simulate sky.
            let len = ray.direction.x * ray.direction.x
                + ray.direction.y * ray.direction.y
                + ray.direction.z * ray.direction.z;
            let unit_vector = ray.direction / len;
            let t = 0.5 * (unit_vector.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn write_color(col: Vector3<f64>, sampples_per_pixel: i32, out_buffer: &mut String) {
    let mut r = col.x;
    let mut g = col.y;
    let mut b = col.z;
    
    let scale = 1.0 / sampples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let color_buff = format!("{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as i32, (256.0 * clamp(g, 0.0, 0.999)) as i32, (256.0 * clamp(b, 0.0, 0.999)) as i32);
    out_buffer.push_str( &color_buff);
}

fn random_double_range(min: f64, max: f64) -> f64{
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return  min;
    }
    if x > max {
        return  max;
    }
    x
}

fn get_random_vec() -> Vector3<f64> {

    let random_min = -1.0;
    let random_max = 1.0;
    let mut p: Vector3<f64>;
    loop {
        p = Vector3::new(random_double_range(random_min, random_max), random_double_range(random_min, random_max), random_double_range(random_min, random_max));
        if p.magnitude2() >= 1.0 {
            continue;
        }
        return p;
    }
}

fn get_random_unit_vec() -> Vector3<f64> {
    let vec = get_random_vec();
    vec / vec.magnitude()
}

fn get_vec_in_hempisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let vec = get_random_unit_vec();
    if cgmath::dot(vec, normal) > 0.0 {
        return vec;
    } else {
        return -vec;
    }
}
