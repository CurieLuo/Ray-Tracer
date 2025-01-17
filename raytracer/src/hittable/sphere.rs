use crate::{hittable::*, material::Material, pdf::onb::ONB, utility::*};

pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let theta = (-p.y).acos();
    let phi = f64::atan2(-p.z, p.x) + PI;
    *u = phi / TAU;
    *v = theta / PI;
}

#[derive(Clone)]
pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(cen: &Vec3, r: f64, material: M) -> Self {
        Sphere {
            center: *cen,
            radius: r,
            mat_ptr: material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction_borrow().length_squared();
        let half_b = dot(r.direction_borrow(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let mut root: f64 = (-half_b - discriminant.sqrt()) / a; //nearest
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let mut rec = HitRecord {
            p: r.at(root),
            normal: Default::default(),
            t: root,
            u: 0.,
            v: 0.,
            front_face: false,
            mat_ptr: &self.mat_ptr,
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::center_radius_new(&self.center, self.radius);
        true
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if self.hit(&Ray::new(o, v, 0.), 0.001, INFINITY).is_none() {
            return 0.;
        }

        let cos_theta_max =
            (1. - self.radius * self.radius / (self.center - *o).length_squared()).sqrt();
        let solid_angle = TAU * (1. - cos_theta_max);
        1. / solid_angle
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(&direction);
        uvw.local_vec(&Vec3::random_to_sphere(self.radius, distance_squared))
    }
}

#[derive(Clone)]
pub struct MovingSphere<M: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat_ptr: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(cen0: &Vec3, cen1: &Vec3, r: f64, time0: f64, time1: f64, material: M) -> Self {
        MovingSphere {
            center0: *cen0,
            center1: *cen1,
            radius: r,
            time0,
            time1,
            mat_ptr: material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction_borrow().length_squared();
        let half_b = dot(r.direction_borrow(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let mut root: f64 = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let mut rec = HitRecord {
            p: r.at(root),
            normal: Default::default(),
            t: root,
            u: 0.,
            v: 0.,
            front_face: false,
            mat_ptr: &self.mat_ptr,
        };
        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::center_radius_new(&(self.center(time0)), self.radius);
        let box1 = AABB::center_radius_new(&(self.center(time1)), self.radius);
        *output_box = surrounding_box(&box0, &box1);
        true
    }
}
