use crate::hittable::*;

#[derive(Clone)]
pub struct Translate<H: Hittable> {
    pub ptr: H,
    pub offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(ptr: H, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(mut output_box) = self.ptr.bounding_box(time0, time1) {
            output_box.min += self.offset;
            output_box.max += self.offset;
            Some(output_box)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct RotateY<H: Hittable> {
    pub ptr: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(ptr: H, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        let mut bbox = ptr.bounding_box(0., 1.);
        //time0 = 0., time1 = 1.
        if let Some(box_) = bbox {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * box_.max.x + (1 - i) as f64 * box_.min.x;
                        let y = j as f64 * box_.max.y + (1 - j) as f64 * box_.min.y;
                        let z = k as f64 * box_.max.z + (1 - k) as f64 * box_.min.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;
                        let tester = Vec3::new(new_x, y, new_z);

                        for c in 0..3 {
                            *min.at(c) = min.get(c).min(tester.get(c));
                            *max.at(c) = max.get(c).max(tester.get(c));
                        }
                    }
                }
            }
            bbox = Some(Aabb::new(min, max));
        }

        Self {
            ptr,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;
        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;
        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}

#[derive(Clone)]
pub struct FlipFace<H: Hittable> {
    ptr: H,
}

impl<H: Hittable> FlipFace<H> {
    pub fn new(ptr: H) -> Self {
        Self { ptr }
    }
}

impl<H: Hittable> Hittable for FlipFace<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(time0, time1)
    }
}
