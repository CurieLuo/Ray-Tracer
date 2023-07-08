use crate::{aabb::*, hittable::*, hittable_list::*, utility::*};

// random int in [min,max)
fn randint(min: i32, max: i32) -> i32 {
    randrange(min as f64, max as f64) as i32
    // rand::thread_rng().gen_range(min..max)
}

#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box_: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::build(
            &mut list.objects.clone(),
            0,
            list.objects.len(),
            time0,
            time1,
        )
    }
    pub fn build(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left;
        let right;
        let axis = randint(0, 3);
        let object_span = end - start;
        if object_span == 1 {
            left = objects[0].clone();
            right = left.clone();
        } else {
            //? unwrap might cause error
            objects[start..end].sort_by_cached_key(|x| {
                x.bounding_box(time0, time1).unwrap().mn().get(axis) as i32
            });
            //sort_unstable_by_key
            //TODO partial comparator
            if object_span == 2 {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                let mid = start + object_span / 2;
                left = Arc::new(Self::build(objects, start, mid, time0, time1));
                right = Arc::new(Self::build(objects, mid, end, time0, time1));
            }
        }
        //? unwrap might cause error
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        Self {
            left,
            right,
            box_: surrounding_box(&box_left, &box_right),
        }
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
        if let Some(hit_left) = self.left.hit(r, t_min, t_max) {
            t_max = hit_left.t;
            rec = Some(hit_left);
        }
        if let Some(hit_right) = self.left.hit(r, t_min, t_max) {
            rec = Some(hit_right);
        }
        rec
    }
}
