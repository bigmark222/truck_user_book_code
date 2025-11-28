use truck_meshalgo::prelude::*;
use truck_meshes::write_polygon_mesh;

fn main() {
    // Load the generated sphere OBJ from output/ embedded at compile time
    let mut mesh: PolygonMesh =
        obj::read(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/output/sphere.obj")).as_slice())
            .unwrap();
    println!("default shell condition: {:?}", mesh.shell_condition());

    // Merge duplicate vertices within 1e-3
    mesh.put_together_same_attrs(1.0e-3);
    println!("after merge: {:?}", mesh.shell_condition());

    // Flat normals for a faceted look
    mesh.add_naive_normals(true);
    write_polygon_mesh(&mesh, "output/mirror-ball.obj");

    // Smooth normals for softer shading
    mesh.add_smooth_normals(1.0, true); // ~57° crease angle
    mesh.normalize_normals(); // keep normal lengths unit after any edits/imports
    write_polygon_mesh(&mesh, "output/mirror-ball-with-smooth-normal.obj");
}
