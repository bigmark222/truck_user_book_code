use truck_meshalgo::prelude::NormalFilters;

fn main() {
    let mut mesh = truck_meshes::icosahedron();
    mesh.add_naive_normals(true); // optional, for shading
    truck_meshes::write_polygon_mesh(&mesh, "output/icosahedron.obj");
}
