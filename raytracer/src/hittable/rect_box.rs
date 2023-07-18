use crate::{aarect::*, hittable::*, hittable_list::*};

pub struct RectBox {
    pub bbox: Aabb,
    pub sides: HittableList,
}

impl RectBox {
    pub fn new<M: Material + Clone + Copy + 'static>(p0: Point3, p1: Point3, ptr: M) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr)));
        sides.add(Arc::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr)));
        sides.add(Arc::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr)));
        sides.add(Arc::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr)));
        sides.add(Arc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr)));
        sides.add(Arc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr)));

        Self {
            bbox: Aabb::new(p0, p1),
            sides,
        }
    }
}

impl Hittable for RectBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
