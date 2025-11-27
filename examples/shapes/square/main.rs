fn main() {
    let mesh = truck_meshes::square();
    truck_meshes::write_polygon_mesh(&mesh, "output/square.obj");
}