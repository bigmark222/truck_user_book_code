use truck_meshalgo::prelude::*;

pub fn write_polygon_mesh(mesh: &PolygonMesh, path: &str) {
    let mut obj = std::fs::File::create(path).unwrap();
    obj::write(mesh, &mut obj).unwrap();
}

pub mod shapes;
pub use shapes::{
    triangle,
    square,
    tetrahedron,
    hexahedron,
    octahedron,
    dodecahedron,
    icosahedron,
};

pub mod utils;
pub use utils::normal_helpers::{
    compute_face_normals,
    compute_vertex_normals,
    add_face_normals,
    add_vertex_normals,
    normalize_vertex_normals,
};
