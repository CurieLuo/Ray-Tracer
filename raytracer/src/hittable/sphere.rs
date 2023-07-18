use crate::{hittable::*, onb::*};

pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin (outward_normal).
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    // <1 0 0> yields <0.50 0.50> <-1 0 0> yields <0.00 0.50>
    // <0 1 0> yields <0.50 1.00> < 0 -1 0> yields <0.50 0.00>
    // <0 0 1> yields <0.25 0.50> < 0 0 -1> yields <0.75 0.50>
    let theta = (-p.y).acos();
    let phi = f64::atan2(-p.z, p.x) + PI;
    let u = phi / (2. * PI);
    let v = theta / PI;
    (u, v)
}

#[derive(Clone, Copy)]
pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat_ptr: M) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mat_ptr = &self.mat_ptr;
        let (u, v) = get_sphere_uv(outward_normal);
        let mut rec = HitRecord::new(root, p, mat_ptr, u, v);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if self.hit(&Ray::new(o, v, 0.), 0.001, INFINITY).is_none() {
            return 0.;
        }
        let cos_theta_max = (1. - self.radius.powi(2) / (self.center - o).length_squared()).sqrt();
        let solid_angle = 2. * PI * (1. - cos_theta_max);
        1. / solid_angle
    }
    fn random(&self, o: Point3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(direction);
        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}

#[derive(Clone, Copy)]
pub struct MovingSphere<M: Material> {
    pub center0: Point3,
    // pub move_dir: Vec3,
    pub center1: Point3,
    pub time0: f64,
    //pub time_tot: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: M,
    ) -> Self {
        Self {
            center0,
            //move_dir: center1 - center0,
            center1,
            time0,
            //time_tot: time1 - time0,
            time1,
            radius,
            mat_ptr,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center0 //+ (time - self.time0) / self.time_tot * self.move_dir
         + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center(r.time)) / self.radius;
        let mat_ptr = &self.mat_ptr;
        let (u, v) = get_sphere_uv(outward_normal);
        let mut rec = HitRecord::new(root, p, mat_ptr, u, v);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(&box0, &box1))
    }
    // TODO pdf related methods
}