use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// Equilateral tetrahedron (triangular pyramid).
pub fn tetrahedron() -> PolygonMesh {

    let positions = vec![
        Point3::new(0.0, 0.0, 0.0), // base 1 [0]
        Point3::new(1.0, 0.0, 0.0), // base 2 [1]
        Point3::new(0.5, f64::sqrt(3.0) / 2.0, 0.0), // base 3 [2]
        Point3::new(0.5, f64::sqrt(3.0) / 6.0, f64::sqrt(6.0) / 3.0), // apex [3]
    ];

    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };

    let faces = Faces::from_iter([
        [2, 1, 0], // base
        [0, 1, 3], // side 1
        [1, 2, 3], // side 2
        [2, 0, 3], // side 3
    ]);

    PolygonMesh::new(attrs, faces)

}


