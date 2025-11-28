use truck_meshes::{
    icosahedron,
    add_face_normals,
    add_vertex_normals,
    normalize_vertex_normals,
    write_polygon_mesh,
};

fn main() {
    // Flat shading: one normal per triangle
    let mut flat = icosahedron();
    add_face_normals(&mut flat);
    write_polygon_mesh(&flat, "output/icosahedron_flat.obj");

    // Smooth shading: blended normals per vertex
    let mut smooth = icosahedron();
    add_vertex_normals(&mut smooth);
    normalize_vertex_normals(&mut smooth); // keep them unit length
    write_polygon_mesh(&smooth, "output/icosahedron_smooth.obj");
}
