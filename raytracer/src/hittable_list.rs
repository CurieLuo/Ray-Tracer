use crate::{aabb::*, hittable::*, utility::*};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec.clone());
            }
        }

        rec
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut first_box = true;
        let mut output_box = Aabb::default();
        for object in self.objects.iter() {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    first_box = false;
                    temp_box
                } else {
                    surrounding_box(&output_box, &temp_box)
                };
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}
