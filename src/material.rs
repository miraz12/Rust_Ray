use cgmath::{Vector3, Zero, InnerSpace};
use rand::Rng;

use crate::{hittablelist::HitRecord, ray::Ray, get_random_in_unit_sphere};


pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f64>
}

impl Material for Lambertian{
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut scatter_direction = rec.normal + get_random_in_unit_sphere();
        if scatter_direction.is_zero() {
            scatter_direction = rec.normal;
        }
        let scettered = Ray::new(rec.p, scatter_direction);
        Some((scettered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64
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
       let scattered = Ray::new(rec.p, reflected + self.fuzz*get_random_in_unit_sphere());
       if cgmath::dot(scattered.direction, rec.normal) > 0.0 {
           return Some((scattered, self.albedo));
       } else {
           return None;
       }
    }
}

pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
        let mut cos_theta = cgmath::dot(-uv, n);
        if cos_theta > 1.0 {
           cos_theta = 1.0; 
        }
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.magnitude2()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))

    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = 1.0/self.ir;
        } else {
            refraction_ratio = self.ir;
        }
        let unit_direction = ray.direction/ray.direction.magnitude();
        let mut cos_theta = cgmath::dot(-unit_direction, rec.normal);
        if cos_theta > 1.0 {
           cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

        let direction: Vector3<f64>;

        let mut rng = rand::thread_rng();
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            direction = Metal::reflect(unit_direction, rec.normal);
        } else {
            direction = Dielectric::refract(unit_direction, rec.normal, refraction_ratio)
        }
        let scattered = Ray::new(rec.p, direction);
        
        Some((scattered, attenuation))
    }
}
