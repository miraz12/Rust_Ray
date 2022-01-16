use cgmath::Vector3;
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

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let samples_per_pixel = 100;

    let cam = Camera::new();

    let mut world = HittableList::default();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let mut out_buffer: String = format!("P3\n{} {}\n255\n", image_width, image_height);

   
    

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _k in 0..samples_per_pixel {
                let u = ((i as f64 + random_double_range(0.0, 1.0)) / (image_width as f64 - 1.0)) as f64;
                let v = ((j as f64 + random_double_range(0.0, 1.0) )/ (image_height as f64 - 1.0)) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color_world(r, &world);
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

fn write_color(col: Vector3<f64>, sampples_per_pixel: i32, out_buffer: &mut String) {
    let mut r = col.x;
    let mut g = col.y;
    let mut b = col.z;
    
    let scale = 1.0 / sampples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

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