use cgmath::{Vector3, InnerSpace};
use crate::{ray::Ray, random_double_range};


pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    lens_radius: f64,
}

fn get_random_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.magnitude2() >= 1.0 {
            continue;
        }
        return p;
    }
}


impl Camera {
    pub fn new (lookfrom: Vector3<f64>, loolat: Vector3<f64>, vup: Vector3<f64>, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera{
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h; 
        let viewport_width = aspect_ratio * viewport_height;

        let mut w = lookfrom - loolat;
        w = w/w.magnitude();
        let mut u = vup.cross(w);
        u = u/u.magnitude();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius}
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        let rd = self.lens_radius * get_random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin+offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }
    
}