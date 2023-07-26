#![allow(dead_code, unused_imports)]

use crate::{
    bvh::*, hittable_list::*, material::generic::*, material::*, texture::*, triangle::*,
    utility::*,
};
use ndarray::{array, Array2};
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
    for model in models {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let indices = &mesh.indices;
        let texcoords = &mesh.texcoords;
        let texcoord_indices = &mesh.texcoord_indices;
        let normals = &mesh.normals;
        let normal_indices = &mesh.normal_indices;
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
            faces.add(Box::new(Triangle::new(
                (points[idx0], points[idx1], points[idx2]),
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
                (n[0], n[1], n[2]),
                None,
            )));
        }
        objects.add(Box::new(BvhNode::new(faces, 0., 1.)));
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

    let nmap = ImageTexture::new(noral_map_name);
    let mut objects = HittableList::new();
    for model in models {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let indices = &mesh.indices;
        let texcoords = &mesh.texcoords;
        let texcoord_indices = &mesh.texcoord_indices;
        let normals = &mesh.normals;
        let normal_indices = &mesh.normal_indices;
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
            faces.add(Box::new(Triangle::new(
                (points[idx0], points[idx1], points[idx2]),
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
                (n[0], n[1], n[2]),
                Some(nmap.clone()),
            )));
        }
        objects.add(Box::new(BvhNode::new(faces, 0., 1.)));
    }
    objects
}

fn parse(color_str: &str) -> Vec<f64> {
    color_str
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<f64>>()
}
pub fn load_obj_and_mtl(
    path: &str,
    short_file_name: &str,
    scale: f64,
    rot: Array2<f64>, // rotation
    shift: Vec3,
) -> HittableList {
    let (models, materials) = tobj::load_obj(
        path.to_owned() + short_file_name,
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load .obj file.");
    let materials: Vec<tobj::Material> = materials.unwrap();
    // println!("{:#?}", materials);
    let mut objects = HittableList::new();
    for model in models {
        let mesh = &model.mesh;
        let material = &materials[mesh.material_id.unwrap()];
        let nmap = material
            .normal_texture
            .as_ref()
            .map(|nmap_name| ImageTexture::new((path.to_owned() + nmap_name).as_str()));
        let diffuse = if let Some(diffuse_name) = &material.diffuse_texture {
            MappingTexture::Texture(ImageTexture::new((path.to_owned() + diffuse_name).as_str()))
        } else {
            MappingTexture::Color(Color::from_array(
                material.diffuse.as_ref().unwrap_or(&[0.; 3]),
            ))
        };
        let specular = if let Some(specular_name) = &material.specular_texture {
            MappingTexture::Texture(ImageTexture::new(
                (path.to_owned() + specular_name).as_str(),
            ))
        } else if let Some(specular_color) = &material.specular {
            MappingTexture::Color(Color::from_array(specular_color))
        } else {
            diffuse.clone()
        };
        let rough = if let Some(shine_name) = &material.shininess_texture {
            MappingTexture::Texture(GreyImageTexture::new(
                (path.to_owned() + shine_name).as_str(),
            ))
        } else {
            MappingTexture::Color(Color::new(
                1. - material.shininess.unwrap_or(96.078431) / 1000.,
                0.,
                0.,
            ))
            // TODO default value ?????????
        };
        let alpha = material.dissolve.unwrap_or(1.);
        let optical_density = material.optical_density.unwrap_or(1.);
        let emit_texture = material.unknown_param.get("map_Ke");
        let emit_color = material.unknown_param.get("Ke");
        let emit = if let Some(emit_name) = emit_texture {
            MappingTexture::Texture(ImageTexture::new((path.to_owned() + emit_name).as_str()))
        } else {
            MappingTexture::Color(Color::from_array(
                &emit_color.map_or([0.; 3], |color_str| {
                    parse(color_str).as_slice().try_into().unwrap()
                }),
            ))
        };

        let mat = Generic::new(diffuse, specular, emit, rough, alpha, optical_density);

        let positions = &mesh.positions;
        let indices = &mesh.indices;
        let texcoords = &mesh.texcoords;
        let texcoord_indices = &mesh.texcoord_indices;
        let normals = &mesh.normals;
        let normal_indices = &mesh.normal_indices;
        let mut points = Vec::new();
        let mut faces = HittableList::new();
        for pos in positions.chunks_exact(3) {
            points.push(matmul(&rot, Point3::new(pos[0], pos[1], pos[2])) * scale + shift);
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
                    n[j] = matmul(
                        &rot,
                        Vec3::new(
                            normals[nidx * 3],
                            normals[nidx * 3 + 1],
                            normals[nidx * 3 + 2],
                        ),
                    );
                }
            }
            faces.add(Box::new(Triangle::new(
                (points[idx0], points[idx1], points[idx2]),
                mat.clone(),
                uv[0],
                uv[1],
                uv[2],
                (n[0], n[1], n[2]),
                nmap.clone(),
            )));
        }
        objects.add(Box::new(BvhNode::new(faces, 0., 1.)));
    }
    if objects.len() > 3 {
        let mut tmp = HittableList::new();
        tmp.add(Box::new(BvhNode::new(objects, 0., 1.)));
        return tmp;
    }
    objects
}
