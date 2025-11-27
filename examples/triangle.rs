fn main() {
    let mesh = truck_meshes::triangle();
    truck_meshes::write_polygon_mesh(&mesh, "output/triangle.obj");
}