use truck_meshalgo::prelude::*;

/// Returns one normalized face normal per polygon.
pub fn compute_face_normals(mesh: &PolygonMesh) -> Vec<Vector3> {
    let positions = mesh.positions();
    mesh.face_iter()
        .map(|face| {
            // Need at least three vertices to form a plane
            if face.len() < 3 {
                return Vector3::zero();
            }
            // First three positions define the face plane
            let p0 = positions[face[0].pos];
            let p1 = positions[face[1].pos];
            let p2 = positions[face[2].pos];
            let n = (p1 - p0).cross(p2 - p0);
            // Skip degenerate/collinear faces
            if n.magnitude2() < 1e-12 {
                return Vector3::zero();
            }
            n.normalize()
        })
        .collect()
}

/// Returns one normalized vertex normal per vertex (averaged from faces).
pub fn compute_vertex_normals(mesh: &PolygonMesh) -> Vec<Vector3> {
    let positions = mesh.positions();
    let face_normals = compute_face_normals(mesh);

    let mut vertex_normals = vec![Vector3::zero(); positions.len()];
    for (face_idx, face) in mesh.face_iter().enumerate() {
        let n = face_normals[face_idx];
        // Accumulate adjacent face normals onto each vertex
        for v in face {
            vertex_normals[v.pos] += n;
        }
    }
    // Normalize summed vectors so each vertex normal is unit length
    for n in vertex_normals.iter_mut() {
        *n = n.normalize();
    }
    vertex_normals
}

/// Computes and attaches a normal per face, wiring each vertex in the face to that normal.
pub fn add_face_normals(mesh: &mut PolygonMesh) {
    let normals = compute_face_normals(mesh);
    let editor = mesh.editor();
    editor.attributes.normals = normals;
    for (idx, face) in editor.faces.face_iter_mut().enumerate() {
        for v in face.iter_mut() {
            v.nor = Some(idx);
        }
    }
}

/// Computes and attaches a normal per vertex, wiring indices to match position indices.
pub fn add_vertex_normals(mesh: &mut PolygonMesh) {
    let normals = compute_vertex_normals(mesh);
    let editor = mesh.editor();
    editor.attributes.normals = normals;
    for face in editor.faces.face_iter_mut() {
        for v in face.iter_mut() {
            v.nor = Some(v.pos);
        }
    }
}

/// Normalizes any normals currently stored on the mesh in-place.
pub fn normalize_vertex_normals(mesh: &mut PolygonMesh) {
    let editor = mesh.editor();
    for n in editor.attributes.normals.iter_mut() {
        *n = n.normalize();
    }
}
