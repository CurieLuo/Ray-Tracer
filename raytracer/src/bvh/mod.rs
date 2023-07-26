use crate::{hittable::*, hittable_list::*, utility::*};
use aabb::*;
pub mod aabb;

// #[derive(Clone)]
pub struct BvhNode {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub box_: Aabb,
} //bounding volume hierachy, similar to a k-D tree

impl BvhNode {
    pub fn new(list: HittableList, time0: f64, time1: f64) -> Self {
        Self::build(list.objects, time0, time1)
    }
    pub fn build(mut objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let left;
        let right;
        let axis = randint(0, 3) as usize;
        let object_span = objects.len();
        if object_span == 0 {
            panic!("BVH: Empty list");
        }
        if object_span == 1 {
            left = Some(objects.remove(0));
            right = None;
        } else {
            //? unwrap might cause error
            objects.sort_unstable_by(|a, b| {
                let ka = a.bounding_box(time0, time1).unwrap().min[axis];
                let kb = b.bounding_box(time0, time1).unwrap().min[axis];
                f64::partial_cmp(&ka, &kb).unwrap()
            });
            if object_span == 2 {
                right = Some(objects.remove(1));
                left = Some(objects.remove(0));
            } else {
                let objects2 = objects.split_off(object_span / 2);
                left = Some(Box::new(Self::build(objects, time0, time1)));
                right = Some(Box::new(Self::build(objects2, time0, time1)));
            }
        }
        //? unwrap might cause error
        //should use Option<>, where None stands for infinity (in R^3 space)
        let box_left = left.as_ref().unwrap().bounding_box(time0, time1).unwrap();
        let box_ = if right.is_some() {
            let box_right = right.as_ref().unwrap().bounding_box(time0, time1).unwrap();
            surrounding_box(&box_left, &box_right)
        } else {
            box_left
        };
        Self { left, right, box_ }
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.box_)
    }
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if !self.box_.hit(r, t_min, t_max) {
            return None;
        }
        let mut rec = None;
        if let Some(hit_left) = self.left.as_ref().unwrap().hit(r, t_min, t_max) {
            t_max = hit_left.t;
            rec = Some(hit_left);
        }
        if let Some(right) = self.right.as_ref() {
            if let Some(hit_right) = right.hit(r, t_min, t_max) {
                rec = Some(hit_right);
            }
        }
        rec
    }
}
