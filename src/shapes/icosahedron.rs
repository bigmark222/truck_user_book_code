use truck_meshalgo::prelude::*;

// Uses dodecahedron() from the same crate

/// Icosahedron via dual of a dodecahedron.
pub fn icosahedron() -> PolygonMesh {
    let dodeca: PolygonMesh = crate::dodecahedron();
    let d_positions = dodeca.positions();

    let positions: Vec<Point3> = dodeca
        .face_iter()
        .map(|face| {
            let centroid = face
                .iter()
                .map(|v| d_positions[v.pos].to_vec())
                .sum::<Vector3>();
            Point3::from_vec(centroid.normalize())
        })
        .collect();

    let mut faces: Faces = (0..20)
        .map(|i| {
            dodeca
                .face_iter()
                .enumerate()
                .filter(|(_, f)| f.contains(&i.into()))
                .map(|(idx, _)| idx)
                .collect::<Vec<usize>>()
        })
        .collect();

    faces.face_iter_mut().for_each(|face| {
        let p: Vec<Point3> = face.iter().map(|v| positions[v.pos]).collect();
        let center = p[0].to_vec() + p[1].to_vec() + p[2].to_vec();
        let normal = (p[1] - p[0]).cross(p[2] - p[0]).normalize();
        if center.dot(normal) < 0.0 {
            face.swap(0, 1);
        }
    });

    PolygonMesh::new(
        StandardAttributes {
            positions,
            ..Default::default()
        },
        faces,
    )


}
