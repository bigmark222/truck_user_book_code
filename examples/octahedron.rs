fn main() {
    let mesh = truck_meshes::octahedron();
    truck_meshes::write_polygon_mesh(&mesh, "output/octahedron.obj");
}