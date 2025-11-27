fn main() {
    let mesh = truck_meshes::tetrahedron();
    truck_meshes::write_polygon_mesh(&mesh, "output/tetrahedron.obj");
}