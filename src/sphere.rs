use cgmath::{InnerSpace, Vector3};
use crate::{material::Material, ray::Ray, hittablelist::{HitRecord, Hittable}};

pub struct Sphere<T: Material>{
    pub radius: f64,
    pub center: Vector3<f64>,
    pub material: T
}

impl<T: Material> Sphere<T>{
    pub fn new(center: Vector3<f64>, radius: f64, material: T) -> Sphere<T> {
        Sphere::<T> { center, radius, material}
    }
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = cgmath::dot(oc, ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord {
            p: ray.at(root),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
            material: &self.material
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Some(rec)
    }
}
