use crate::ray::{Hittable, Ray, HitRecord};

struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + 'static>>
}

impl HittableList {
    pub fn add(&mut self, obj: impl Hittable + Send + 'static) {
        self.objects.push(Box::new(obj as Box<dyn Hittable + Send>));
    }
}

 
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut
        false
    }
}