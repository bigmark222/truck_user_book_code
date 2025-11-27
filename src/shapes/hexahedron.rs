use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// Unit cube (hexahedron) using quads.
pub fn hexahedron() -> PolygonMesh {

    let positions = vec![
        Point3::new(0.0, 0.0, 0.0), //[0]
        Point3::new(1.0, 0.0, 0.0), //[1]
        Point3::new(1.0, 1.0, 0.0), //[2]
        Point3::new(0.0, 1.0, 0.0), //[3]
        Point3::new(0.0, 0.0, 1.0), //[4]
        Point3::new(1.0, 0.0, 1.0), //[5]
        Point3::new(1.0, 1.0, 1.0), //[6]
        Point3::new(0.0, 1.0, 1.0), //[7]
    ];

    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };

    let faces = Faces::from_iter([
        [3, 2, 1, 0], // bottom
        [0, 1, 5, 4], // front
        [1, 2, 6, 5], // right
        [2, 3, 7, 6], // back
        [3, 0, 4, 7], // left
        [4, 5, 6, 7], // top
    ]);

    PolygonMesh::new(attrs, faces)

}
