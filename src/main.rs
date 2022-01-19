use cgmath::{Vector3, InnerSpace, ElementWise};
use std::{io::Write};
use rand::prelude::*;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;


mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod hittablelist;
use hittablelist::{HittableList, Hittable};
mod camera;
use camera::Camera;

use crate::material::{Lambertian, Metal, Dielectric};
mod material;

fn main() {

    let world: Arc<HittableList> = Arc::new(setup_scene());

    let aspect_ratio = 3.0 / 2.0;
    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 0.1, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Arc::new(Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus));

    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    

    let vec_list: Arc<Mutex<Vec<Vec<Vector3<i32>>>>> = Arc::new(Mutex::new(Vec::<Vec::<Vector3<i32>>>::new()));
    //let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    //let mut t;
    // TODO: Threading

    for i in 0..1 {
        let threadWorld = world.clone();
        let threadCam = cam.clone();
       // t = thread::spawn(move || { 
        //    let list = &vec_list;
            let col = shoot_rays(image_width, image_height, &threadWorld, &threadCam);
            vec_list.lock().unwrap().push(col);
       // });
    }

    let file = match std::fs::File::create("image.ppm") {
        Err(why) => panic!("Couldn't create file: {}", why),
        Ok(file) => file,
    };
    match write!(&file, "P3\n{} {}\n255\n", image_width, image_height) {
        Err(why) => println!("Couldn't write file {}", why),
        _ => (),
    }
    for a in vec_list.lock().unwrap().iter() {
        for v in a {
            match writeln!(&file, "{} {} {}", v.x, v.y, v.z) {
                Err(why) => println!("Couldn't write file {}", why),
                _ => ()
            };
        }
    }
}

fn shoot_rays(image_width: i32, image_height: i32, world: &HittableList, cam: &Camera) -> Vec<Vector3<i32>>{

    let mut color_vec = Vec::new();
    let samples_per_pixel = 100;
    let max_depth = 50;

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _k in 0..samples_per_pixel {
                let u = ((i as f64 + random_double_range(0.0, 1.0)) / (image_width as f64 - 1.0)) as f64;
                let v = ((j as f64 + random_double_range(0.0, 1.0) )/ (image_height as f64 - 1.0)) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color_world(r, world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel, &mut color_vec );
        }
    }
    eprintln!("Done.");
    color_vec
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
                    return ret.1.mul_element_wise(bounce);
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

fn write_color(col: Vector3<f64>, sampples_per_pixel: i32, out_buffer: &mut Vec<Vector3<i32>>) {
    let mut r = col.x;
    let mut g = col.y;
    let mut b = col.z;
    
    let scale = 1.0 / sampples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    out_buffer.push(Vector3::new(
        (256.0 * clamp(r, 0.0, 0.999)) as i32, 
        (256.0 * clamp(g, 0.0, 0.999)) as i32, 
        (256.0 * clamp(b, 0.0, 0.999)) as i32));
}

fn random_double_range(min: f64, max: f64) -> f64{
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}

fn random_double() -> f64{
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
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

fn get_random_in_unit_sphere() -> Vector3<f64> {

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

fn get_random_vec() -> Vector3<f64> {
    Vector3::new(random_double(), random_double(), random_double())
}

fn get_random_vec_range(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
}

fn setup_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground = Lambertian {albedo: Vector3::new(0.5, 0.5, 0.5)};
    world.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, ground));

    for a in -11..11 {
        for b in -11..11  {
            let choose_mat = random_double();
            let center = Vector3::new(a as f64 + 0.5 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                
                if choose_mat < 0.8{
                    let albedo = get_random_vec().mul_element_wise(get_random_vec());
                    let sphere_material = Lambertian {albedo};
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = get_random_vec_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal {albedo, fuzz};                   
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Dielectric { ir: 1.5};
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric{ir: 1.5};
    let material2 = Lambertian{albedo: Vector3::new(0.4, 0.2, 0.1)};
    let material3 = Metal{ albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0};

    world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
