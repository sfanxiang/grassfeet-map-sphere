use icosahedron::{ArraySerializedVector, Polyhedron};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

fn write(
    file: &str,
    edges: &Vec<BTreeSet<u32>>,
    positions: &Vec<ArraySerializedVector>,
    normals: &Vec<ArraySerializedVector>,
    groups: &Vec<u32>,
    group_positions: &Vec<ArraySerializedVector>,
    group_normals: &Vec<ArraySerializedVector>,
) -> std::io::Result<()> {
    let mut f = File::create(file)?;

    // Write magic & version
    f.write_all(&[b'G', b'R', b'A', b'S', b'S', b'F', b'E', b'E', b'T', 2])?;

    // Write total vertices
    f.write_all(&(edges.len() as u32).to_be_bytes())?;

    // Write total groups
    f.write_all(&(group_positions.len() as u32).to_be_bytes())?;

    for i in 0..edges.len() {
        // Write positions
        f.write(&(positions[i].0.x as f32).to_bits().to_be_bytes())?;
        f.write(&(positions[i].0.y as f32).to_bits().to_be_bytes())?;
        f.write(&(positions[i].0.z as f32).to_bits().to_be_bytes())?;

        // Write normals
        f.write(&(normals[i].0.x as f32).to_bits().to_be_bytes())?;
        f.write(&(normals[i].0.y as f32).to_bits().to_be_bytes())?;
        f.write(&(normals[i].0.z as f32).to_bits().to_be_bytes())?;

        // Write group
        f.write(&groups[i].to_be_bytes())?;

        // Write connected edges
        f.write(&(edges[i].len() as u32).to_be_bytes())?;
        for next in &edges[i] {
            f.write(&(*next as u32).to_be_bytes())?;
        }
    }

    for i in 0..group_positions.len() {
        // Write positions
        f.write(&(group_positions[i].0.x as f32).to_bits().to_be_bytes())?;
        f.write(&(group_positions[i].0.y as f32).to_bits().to_be_bytes())?;
        f.write(&(group_positions[i].0.z as f32).to_bits().to_be_bytes())?;

        // Write normals
        f.write(&(group_normals[i].0.x as f32).to_bits().to_be_bytes())?;
        f.write(&(group_normals[i].0.y as f32).to_bits().to_be_bytes())?;
        f.write(&(group_normals[i].0.z as f32).to_bits().to_be_bytes())?;
    }

    Ok(())
}

fn dist_square(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1)
}

fn main() {
    let mut poly = Polyhedron::new_isocahedron(92.333333333, 6);
    poly.compute_triangle_normals();

    println!("Triangles: {}", poly.cells.len());
    println!("Vertices: {}", poly.positions.len());

    if poly.positions.len() as u64 >= u32::max_value() as u64 {
        eprintln!("Error: Generated array too big.");
        return;
    }

    let mut edges: Vec<BTreeSet<u32>> = Vec::new();
    edges.resize_with(poly.positions.len(), || BTreeSet::new());

    for tri in poly.cells {
        edges[tri.a].insert(tri.b as u32);
        edges[tri.a].insert(tri.c as u32);
        edges[tri.b].insert(tri.a as u32);
        edges[tri.b].insert(tri.c as u32);
        edges[tri.c].insert(tri.a as u32);
        edges[tri.c].insert(tri.b as u32);
    }

    let mut sparse = Polyhedron::new_isocahedron(92.333333333, 4);
    sparse.compute_triangle_normals();
    assert!(sparse.positions.len() < poly.positions.len());
    println!("Sparse vertices: {}", sparse.positions.len());

    let mut groups: Vec<u32> = Vec::new();
    groups.resize(edges.len(), 0);

    for i in 0..edges.len() {
        let mut min_dist = std::f32::MAX;
        for j in 0..sparse.positions.len() {
            let dist = dist_square(
                poly.positions[i].0.x,
                poly.positions[i].0.y,
                poly.positions[i].0.z,
                sparse.positions[j].0.x,
                sparse.positions[j].0.y,
                sparse.positions[j].0.z,
            );
            if dist < min_dist {
                min_dist = dist;
                groups[i] = j as u32;
            }
        }
    }

    if let Err(e) = write(
        "map-sphere.gra",
        &edges,
        &poly.positions,
        &poly.normals,
        &groups,
        &sparse.positions,
        &sparse.normals,
    ) {
        eprintln!("Error: Cannot write, {}", e);
    }
}
