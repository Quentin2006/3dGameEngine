use crate::models::{triangle::Triangle, vec3::Vec3};

/// this fucntion will load in an obj file and return a vector of triangles
///
/// * `file_name`: the name of the file that will be loaded from the object folder
pub fn load_obj_file(file_name: &str) -> Vec<Triangle> {
    let verts = load_verts(file_name);
    load_faces(file_name, verts)
}

/// the first part of an obj file is the vertices, we will load these in memory
///
/// * `file_name`: the name of the file that will be loaded from the object folder
pub fn load_verts(file_name: &str) -> Vec<Vec3> {
    let mut verts: Vec<Vec3> = Vec::new();
    let contents = std::fs::read_to_string(file_name).expect("Unable to open file");

    for line in contents.lines() {
        if !line.is_empty() {
            // if its a vert, read in the next 3 vals
            if line.starts_with('v') {
                let vals: Vec<&str> = line.split_whitespace().collect();

                // start at 1 because we already read in the first val
                let x: f32 = vals[1].parse().unwrap();
                let y: f32 = vals[2].parse().unwrap();
                let z: f32 = vals[3].parse().unwrap();
                verts.push(Vec3 { x, y, z });
            }
        }
    }
    verts
}

pub fn load_faces(file_name: &str, verts: Vec<Vec3>) -> Vec<Triangle> {
    let mut faces: Vec<Triangle> = Vec::new();
    let contents = std::fs::read_to_string(file_name).expect("Unable to open file");

    for line in contents.lines() {
        if line.starts_with('f') {
            // we now have the vals
            let vals: Vec<&str> = line.split_whitespace().collect();
            // we will have an array of 3 vals, each val will correspond to a vert index

            // get all indecies
            let i0: usize = vals[1].parse().unwrap();
            let i1: usize = vals[2].parse().unwrap();
            let i2: usize = vals[3].parse().unwrap();

            faces.push(Triangle {
                v0: (verts[i0 - 1]),
                v1: (verts[i1 - 1]),
                v2: (verts[i2 - 1]),
            })
        }
    }

    faces
}
