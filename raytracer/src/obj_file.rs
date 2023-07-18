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
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load .obj file.");
    let mut objects = HittableList::new();
    for m in models {
        let positions = &m.mesh.positions;
        let indices = &m.mesh.indices;
        let texcoords = &m.mesh.texcoords;
        let texcoord_indices = &m.mesh.texcoord_indices;
        let mut points = Vec::new();
        let mut faces = HittableList::new();
        for pos in positions.chunks_exact(3) {
            points.push(Point3::new(pos[0], pos[1], pos[2]) * scale);
        }
        for i in (0..(indices.len() - 2)).step_by(3) {
            let (idx0, idx1, idx2) = (
                indices[i] as usize,
                indices[i + 1] as usize,
                indices[i + 2] as usize,
            );
            let mut uv0 = (0., 0.);
            let mut uv1 = (1., 0.);
            let mut uv2 = (0., 1.);
            if !texcoords.is_empty() {
                let (tidx0, tidx1, tidx2) = (
                    texcoord_indices[i] as usize,
                    texcoord_indices[i + 1] as usize,
                    texcoord_indices[i + 2] as usize,
                );
                uv0 = (texcoords[tidx0 * 2], texcoords[tidx0 * 2 + 1]);
                uv1 = (texcoords[tidx1 * 2], texcoords[tidx1 * 2 + 1]);
                uv2 = (texcoords[tidx2 * 2], texcoords[tidx2 * 2 + 1]);
            }
            faces.add(Arc::new(Triangle::new(
                points[idx0],
                points[idx1],
                points[idx2],
                mat.clone(),
                uv0,
                uv1,
                uv2,
            )));
        }
        objects.add(Arc::new(BvhNode::new(&faces, 0., 1.)));
    }
    objects
}
