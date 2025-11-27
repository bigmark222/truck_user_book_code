use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// A single equilateral triangle in the XY plane.
pub fn triangle() -> PolygonMesh {

    let positions = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 2.0, 0.0),
    ];

    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };

    let faces = Faces::from_iter([[0, 1, 2]]);

    PolygonMesh::new(attrs, faces)

}
