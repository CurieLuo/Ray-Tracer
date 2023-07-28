use crate::{hittable::*, material::Material, utility::*};

#[derive(Clone, Default)]
pub struct XYRect<M: Material> {
    mp: M,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64, //z = k
}

impl<M: Material> XYRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: M) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: material,
        }
    }
}

impl<M: Material> Hittable for XYRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin_borrow().z) / r.direction_borrow().z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin_borrow().x + t * r.direction_borrow().x;
        let y = r.origin_borrow().y + t * r.direction_borrow().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Default::default(),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            front_face: false,
            mat_ptr: &self.mp,
        };
        let outward_normal = Vec3::new(0., 0., 1.);
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point3::new(self.x0, self.y0, self.k - 0.0001),
            &Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let op = self.hit(&Ray::new(origin, v, 0.), 0.001, INFINITY);
        if op.is_none() {
            return 0.;
        }
        let rec = op.unwrap();
        let area = (self.x1 - self.x0) * (self.y1 - self.y0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (dot(v, &rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let random_point = Point3::new(
            randrange(self.x0, self.x1),
            randrange(self.y0, self.y1),
            self.k,
        );
        random_point - *origin
    }
}

#[derive(Clone, Default)]
pub struct XZRect<M: Material> {
    mp: M,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64, //y = k
}

impl<M: Material> XZRect<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: material,
        }
    }
}

impl<M: Material> Hittable for XZRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin_borrow().y) / r.direction_borrow().y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin_borrow().x + t * r.direction_borrow().x;
        let z = r.origin_borrow().z + t * r.direction_borrow().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Default::default(),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
            mat_ptr: &self.mp,
        };
        let outward_normal = Vec3::new(0., 1., 0.);
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point3::new(self.x0, self.k - 0.0001, self.z0),
            &Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let op = self.hit(&Ray::new(origin, v, 0.), 0.001, INFINITY);
        if op.is_none() {
            return 0.;
        }
        let rec = op.unwrap();
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (dot(v, &rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let random_point = Point3::new(
            randrange(self.x0, self.x1),
            self.k,
            randrange(self.z0, self.z1),
        );
        random_point - *origin
    }
}

#[derive(Clone, Default)]
pub struct YZRect<M: Material> {
    mp: M,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64, //x = k
}

impl<M: Material> YZRect<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: material,
        }
    }
}

impl<M: Material> Hittable for YZRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin_borrow().x) / r.direction_borrow().x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin_borrow().y + t * r.direction_borrow().y;
        let z = r.origin_borrow().z + t * r.direction_borrow().z;
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Default::default(),
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
            mat_ptr: &self.mp,
        };
        let outward_normal = Vec3::new(1., 0., 0.);
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point3::new(self.k - 0.0001, self.y0, self.z0),
            &Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let op = self.hit(&Ray::new(origin, v, 0.), 0.001, INFINITY);
        if op.is_none() {
            return 0.;
        }
        let rec = op.unwrap();
        let area = (self.y1 - self.y0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (dot(v, &rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let random_point = Point3::new(
            self.k,
            randrange(self.y0, self.y1),
            randrange(self.z0, self.z1),
        );
        random_point - *origin
    }
}
