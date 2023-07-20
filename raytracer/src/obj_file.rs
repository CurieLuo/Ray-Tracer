#![allow(dead_code)]
use crate::{
    bvh::*, hittable_list::*, material::*, texture::NormalTexture, triangle::*, utility::*,
};
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
            let mut uv = [(0., 0.), (1., 0.), (0., 1.)];
            if !texcoords.is_empty() {
                for j in 0..3 {
                    let tidx = texcoord_indices[i + j] as usize;
                    uv[j] = (texcoords[tidx * 2], texcoords[tidx * 2 + 1]);
                }
            }
            faces.add(Arc::new(Triangle::new(
                points[idx0],
                points[idx1],
                points[idx2],
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
            )));
        }
        objects.add(Arc::new(BvhNode::new(&faces, 0., 1.)));
    }
    objects
}

pub fn load_obj_and_normal<M: Material + Clone + 'static>(
    file_name: &str,
    noral_map_name: &str,
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
    let nmap = NormalTexture::new(noral_map_name);
    let mut objects = HittableList::new();
    for m in models {
        let positions = &m.mesh.positions;
        let indices = &m.mesh.indices;
        let texcoords = &m.mesh.texcoords;
        let texcoord_indices = &m.mesh.texcoord_indices;
        let normals = &m.mesh.normals;
        let normal_indices = &m.mesh.normal_indices;
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
            let mut uv = [(0., 0.), (1., 0.), (0., 1.)];
            if !texcoords.is_empty() {
                for j in 0..3 {
                    let tidx = texcoord_indices[i + j] as usize;
                    uv[j] = (texcoords[tidx * 2], texcoords[tidx * 2 + 1]);
                }
            }
            let mut n = [cross(points[idx1] - points[idx0], points[idx2] - points[idx0]).unit(); 3];
            if !normals.is_empty() {
                for j in 0..3 {
                    let nidx = normal_indices[i + j] as usize;
                    n[j] = Vec3::new(
                        normals[nidx * 3],
                        normals[nidx * 3 + 1],
                        normals[nidx * 3 + 2],
                    );
                }
            }
            faces.add(Arc::new(TriangleWithNormalMapping::new(
                (points[idx0], points[idx1], points[idx2]),
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
                (n[0], n[1], n[2]),
                nmap.clone(),
            )));
        }
        objects.add(Arc::new(BvhNode::new(&faces, 0., 1.)));
    }
    objects
}
