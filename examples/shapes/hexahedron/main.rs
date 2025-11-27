fn main() {
    let mesh = truck_meshes::hexahedron();
    truck_meshes::write_polygon_mesh(&mesh, "output/hexahedron.obj");
}