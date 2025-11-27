# Truck Meshes

Small Rust crate that builds basic polygon meshes (triangle, square, tetrahedron, hexahedron/cube, octahedron, dodecahedron, icosahedron) using `truck-meshalgo`. Each example writes an OBJ file you can inspect in your favorite viewer.

## Prerequisites
- Rust toolchain (edition 2024; `cargo` available)

## Running
```bash
cargo run --example <shape>
```

Shape names: `triangle`, `square`, `tetrahedron`, `hexahedron`, `octahedron`, `dodecahedron`, `icosahedron`.

Outputs land in `output/<shape>.obj`. For example:
```bash
cargo run --example icosahedron
# writes output/icosahedron.obj
```

## Notes
- Only examples that need extra helpers (e.g., `add_naive_normals` in `examples/icosahedron.rs`) pull in `NormalFilters`; others stick to the crate exports.
- Mesh constructors live in `src/` and are re-exported from `src/lib.rs`, so examples can just call `truck_meshes::<shape>()`.

- **Most of this code is from [Truck Tutorial Code by ricosjp](https://github.com/ricosjp/truck-tutorial-code/tree/v0.6)**