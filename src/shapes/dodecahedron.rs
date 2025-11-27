use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// Dodecahedron built from a cube plus roof vertices.
pub fn dodecahedron() -> PolygonMesh {

    let a = f64::sqrt(3.0) / 3.0; // half cube edge
    let l = 2.0 * a / (1.0 + f64::sqrt(5.0)); // half dodeca edge
    let d = f64::sqrt(1.0 - l * l); // other coordinate by Pythagoras

    let positions = vec![
        Point3::new(-a, -a, -a),
        Point3::new(a, -a, -a),
        Point3::new(a, a, -a),
        Point3::new(-a, a, -a),
        Point3::new(-a, -a, a),
        Point3::new(a, -a, a),
        Point3::new(a, a, a),
        Point3::new(-a, a, a),
        Point3::new(d, -l, 0.0),
        Point3::new(d, l, 0.0),
        Point3::new(-d, l, 0.0),
        Point3::new(-d, -l, 0.0),
        Point3::new(0.0, d, -l),
        Point3::new(0.0, d, l),
        Point3::new(0.0, -d, l),
        Point3::new(0.0, -d, -l),
        Point3::new(-l, 0.0, d),
        Point3::new(l, 0.0, d),
        Point3::new(l, 0.0, -d),
        Point3::new(-l, 0.0, -d),
    ];

    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    
    let faces = Faces::from_iter([
        [4, 14, 5, 17, 16],
        [6, 13, 7, 16, 17],
        [6, 17, 5, 8, 9],
        [4, 16, 7, 10, 11],
        [4, 11, 0, 15, 14],
        [1, 8, 5, 14, 15],
        [6, 9, 2, 12, 13],
        [3, 10, 7, 13, 12],
        [1, 15, 0, 19, 18],
        [1, 18, 2, 9, 8],
        [3, 12, 2, 18, 19],
        [3, 19, 0, 11, 10],
    ]);


    PolygonMesh::new(attrs, faces)

}
