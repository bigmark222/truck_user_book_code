use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// Octahedron with vertices on the coordinate axes.
pub fn octahedron() -> PolygonMesh {

    let positions = vec![
        Point3::new(-1.0, 0.0, 0.0), // (-X) [0]
        Point3::new(1.0, 0.0, 0.0),  // (+X) [1]
        Point3::new(0.0, -1.0, 0.0), // (-Y) [2]
        Point3::new(0.0, 1.0, 0.0),  // (+Y) [3]
        Point3::new(0.0, 0.0, -1.0), // (-Z) [4]
        Point3::new(0.0, 0.0, 1.0),  // (+Z) [5]
    ];

    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };

    let faces = Faces::from_iter([
        [0, 4, 2],
        [2, 4, 1],
        [1, 4, 3],
        [3, 4, 0],
        [0, 2, 5],
        [2, 1, 5],
        [1, 3, 5],
        [3, 0, 5],
    ]);

    PolygonMesh::new(attrs, faces)


}

