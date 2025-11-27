fn main() {
    let mesh = truck_meshes::dodecahedron();
    truck_meshes::write_polygon_mesh(&mesh, "output/dodecahedron.obj");
}