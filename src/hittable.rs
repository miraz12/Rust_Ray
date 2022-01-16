use crate::ray::Hittable;
use crate::ray::Ray;
use crate::ray::HitRecord;

struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut
        false
    }
}