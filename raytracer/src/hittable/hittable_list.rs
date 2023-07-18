use crate::hittable::*;

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
    pub fn _is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
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
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let mut sum = 0.;
        for object in &self.objects {
            sum += object.pdf_value(o, v);
        }
        sum / self.objects.len() as f64
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let int_size = self.objects.len();
        self.objects[randint(0, int_size as i32) as usize].random(o)
    }
}
