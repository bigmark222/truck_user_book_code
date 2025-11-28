use truck_meshalgo::prelude::*;
use truck_meshes::{hexahedron, write_polygon_mesh};

fn main() {
    // --- Step 1: create hexahedron ---
    let hexa = hexahedron();
    // Center the unit cube from the library so projection covers all octants.
    let center = Vector3::new(0.5, 0.5, 0.5);

    // --- Step 2: subdivide each face ---
    const DIVISION: usize = 8;

    // the positions of vertices
    let positions: Vec<Point3> = hexa
        .face_iter()
        .flat_map(|face| {
            // convert face vertex positions into Vec<Vector3>
            let v: Vec<Vector3> = face
                .iter()
                .map(|vertex| (hexa.positions()[vertex.pos] - center).to_vec())
                .collect();

            // create (i,j) grid 0..DIVISION
            (0..=DIVISION)
                .flat_map(move |i| (0..=DIVISION).map(move |j| (i, j)))
                .map(move |(i, j)| {
                    let s = i as f64 / DIVISION as f64;
                    let t = j as f64 / DIVISION as f64;

                    // bilinear interpolation inside the quad
                    v[0] * (1.0 - s) * (1.0 - t)
                        + v[1] * s * (1.0 - t)
                        + v[3] * (1.0 - s) * t
                        + v[2] * s * t
                })
        })
        // project onto the unit sphere
        .map(|vec| Point3::from_vec(vec.normalize()))
        .collect();

    // --- Step 3: compute normals (just position → vector) ---
    let normals: Vec<Vector3> = positions.iter().copied().map(Point3::to_vec).collect();

    // --- Step 4: attributes ---
    let attrs = StandardAttributes {
        positions,
        normals,
        ..Default::default()
    };

    // --- Step 5: face construction ---
    let faces: Faces = (0..6)
        .flat_map(|face_idx| {
            let base = face_idx * (DIVISION + 1) * (DIVISION + 1);

            // closure to map (i,j) → attribute indices
            let to_index = move |i: usize, j: usize| {
                let idx = base + (DIVISION + 1) * i + j;
                // (pos index, texcoord, normal index)
                (idx, None, Some(idx))
            };

            (0..DIVISION)
                .flat_map(move |i| (0..DIVISION).map(move |j| (i, j)))
                .map(move |(i, j)| {
                    [
                        to_index(i, j),
                        to_index(i + 1, j),
                        to_index(i + 1, j + 1),
                        to_index(i, j + 1),
                    ]
                })
        })
        .collect();

    // --- Step 6: build mesh and export ---
    let sphere = PolygonMesh::new(attrs, faces);
    write_polygon_mesh(&sphere, "output/sphere.obj");

    println!("Wrote output/sphere.obj");
}
