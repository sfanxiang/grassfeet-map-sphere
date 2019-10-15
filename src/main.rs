use icosahedron::{ArraySerializedVector, Polyhedron};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

fn write(
    file: &str,
    edges: &Vec<BTreeSet<usize>>,
    positions: &Vec<ArraySerializedVector>,
    normals: &Vec<ArraySerializedVector>,
) -> std::io::Result<()> {
    let mut f = File::create(file)?;

    // Write magic & version
    f.write_all(&[b'G', b'R', b'A', b'S', b'S', b'F', b'E', b'E', b'T', 1])?;

    // Write total vertices
    f.write_all(&(edges.len() as u32).to_be_bytes())?;

    for i in 0..edges.len() {
        // Write positions
        f.write(&(positions[i].0.x as f32).to_bits().to_be_bytes())?;
        f.write(&(positions[i].0.y as f32).to_bits().to_be_bytes())?;
        f.write(&(positions[i].0.z as f32).to_bits().to_be_bytes())?;

        // Write normals
        f.write(&(normals[i].0.x as f32).to_bits().to_be_bytes())?;
        f.write(&(normals[i].0.y as f32).to_bits().to_be_bytes())?;
        f.write(&(normals[i].0.z as f32).to_bits().to_be_bytes())?;

        // Write connected edges
        f.write(&(edges[i].len() as u32).to_be_bytes())?;
        for next in &edges[i] {
            f.write(&(*next as u32).to_be_bytes())?;
        }
    }

    Ok(())
}

fn main() {
    let mut poly = Polyhedron::new_isocahedron(92.333333333, 6);
    poly.compute_triangle_normals();

    println!("Triangles: {}", poly.cells.len());
    println!("Vertices: {}", poly.positions.len());

    if poly.positions.len() as u64 > u32::max_value() as u64 {
        eprintln!("Error: Generated array too big.");
        return;
    }

    let mut edges: Vec<BTreeSet<usize>> = Vec::new();
    edges.resize_with(poly.positions.len(), || BTreeSet::new());

    for tri in poly.cells {
        edges[tri.a].insert(tri.b);
        edges[tri.a].insert(tri.c);
        edges[tri.b].insert(tri.a);
        edges[tri.b].insert(tri.c);
        edges[tri.c].insert(tri.a);
        edges[tri.c].insert(tri.b);
    }

    if let Err(e) = write("map-sphere.gra", &edges, &poly.positions, &poly.normals) {
        eprintln!("Error: Cannot write, {}", e);
    }
}
