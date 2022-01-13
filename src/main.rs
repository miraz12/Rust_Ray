use cgmath::{Vector3, InnerSpace};
use std::io::Write;

mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
 
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

    let file = match std::fs::File::create("image.ppm") {
        Err(why) => panic!("Couldn't create file: {}", why),
        Ok(file) => file,
    };
    match write!(&file, "P3\n{} {}\n255\n", image_width, image_height) {
        Err(why) => println!("Couldn't write file {}", why),
        _ => (),
    }

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = (i as f64 / (image_width as f64 - 1.0)) as f64;
            let v = (j as f64 / (image_height as f64 - 1.0)) as f64;

            let r = Ray::new(
                origin,
                lower_left_cornet + u * horizontal + v * vertical - origin,
            );
            write_color(ray_color(r), &file);
        }
    }
    eprintln!("Done.");
}

fn hit_sphere(center: Vector3<f64>, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.magnitude2();
    let half_b = cgmath::dot(oc, r.direction);
    let c = oc.magnitude2() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
       -1.0 
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Vector3<f64> {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let mut n = ray.at(t) - Vector3::new(0.0, 0.0, -1.0);
        n = n / n.magnitude();
        return 0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let len = ray.direction.x * ray.direction.x
        + ray.direction.y * ray.direction.y
        + ray.direction.z * ray.direction.z;
    let unit_vector = ray.direction / len;
    let t = 0.5 * (unit_vector.y + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn write_color(col: Vector3<f64>, mut file: &std::fs::File) {
    let ir = (255.999 * col.x) as i32;
    let ig = (255.999 * col.y) as i32;
    let ib = (255.999 * col.z) as i32;

    match write!(file, "{} {} {}\n", ir, ig, ib) {
        Err(why) => println!("Couldn't write file {}", why),
        _ => (),
    }
}
