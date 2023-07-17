use crate::{bvh::*, hittable_list::*, material::*, triangle::*, utility::*};
use tobj::{self, LoadOptions};

pub fn load_obj<M: Material + Clone + 'static>(
    file_name: &str,
    mat: M,
    scale: f64,
) -> HittableList {
    let (models, _) = tobj::load_obj(
        file_name,
        &LoadOptions {
            ignore_points: true,
            ignore_lines: true,
            ..LoadOptions::default()
        },
    )
    .expect("Failed to load .obj file.");
    let mut objects = HittableList::new();
    for m in models {
        let positions = &m.mesh.positions;
        let indices = &m.mesh.indices;
        let mut points = Vec::new();
        let mut faces = HittableList::new();
        for i in (0..positions.len()).step_by(3) {
            points.push(Point3::new(positions[i], positions[i + 1], positions[i + 2]) * scale);
        }
        for i in (0..indices.len() - indices.len() % 3).step_by(3) {
            faces.add(Arc::new(Triangle::new(
                points[indices[i] as usize],
                points[indices[i + 1] as usize],
                points[indices[i + 2] as usize],
                mat.clone(),
            )));
        }
        objects.add(Arc::new(BvhNode::new(&faces, 0., 1.)));
    }
    objects
}
