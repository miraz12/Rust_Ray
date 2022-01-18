use cgmath::{Vector3, Zero, InnerSpace};
use crate::{hittablelist::HitRecord, get_random_unit_vec, ray::Ray};


pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f64>
}

impl Material for Lambertian{
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut scatter_direction = rec.normal + get_random_unit_vec();
        if scatter_direction.is_zero() {
            scatter_direction = rec.normal;
        }
        let scettered = Ray::new(rec.p, scatter_direction);
        Some((scettered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f64>
}

impl Metal {
    pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
        v - 2.0 * cgmath::dot(v, n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let unit_vec = ray.direction / ray.direction.magnitude();
       let reflected = Metal::reflect(unit_vec, rec.normal);
       let scattered = Ray::new(rec.p, reflected);
       if cgmath::dot(scattered.direction, rec.normal) > 0.0 {
           return Some((scattered, self.albedo));
       } else {
           return None;
       }
    }
}

