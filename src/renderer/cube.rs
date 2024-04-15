use crate::renderer::terrain::TerrainVertex;

pub fn face_indices(offset: usize) -> Vec<u16> {
    let mut indices: Vec<u16> = Vec::from([
        0, 1, 2, 2, 1, 3
    ]);

    for i in 0..6 {
        indices[i] = indices[i] + (offset * 4) as u16;
    }

    return indices;
}

pub fn face_up(x: usize, y: usize, z: usize, color: [u8; 4], light: u8) -> Vec<TerrainVertex> {
    let mut color = color;
    for i in 0..4 {
        color[i] = color[i] * ((light / 255) as f32) as u8;
    }

    return Vec::from([
        TerrainVertex::new([x as f32, y as f32, (z + 1) as f32], color),
        TerrainVertex::new([(x + 1) as f32, y as f32, (z + 1) as f32], color),
        TerrainVertex::new([x as f32, (y + 1) as f32, (z + 1) as f32], color),
        TerrainVertex::new([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32], color),
    ]);
}

pub fn face_down(x: usize, y: usize, z: usize, color: [u8; 4], light: u8) -> Vec<TerrainVertex> {
    let mut color = color;
    for i in 0..4 {
        color[i] = color[i] * ((light / 255) as f32) as u8;
    }

    return Vec::from([
        TerrainVertex::new([x as f32, y as f32, z as f32], color),
        TerrainVertex::new([x as f32, (y + 1) as f32, z as f32], color),
        TerrainVertex::new([(x + 1) as f32, y as f32, z as f32], color),
        TerrainVertex::new([(x + 1) as f32, (y + 1) as f32, z as f32], color),
    ]);
}

pub fn face_back(x: usize, y: usize, z: usize, color: [u8; 4], light: u8) -> Vec<TerrainVertex> {
    let mut color = color;
    for i in 0..4 {
        color[i] = color[i] * ((light / 255) as f32) as u8;
    }

    return Vec::from([
        TerrainVertex::new([x as f32, (y + 1) as f32, z as f32], color),
        TerrainVertex::new([x as f32, (y + 1) as f32, (z + 1) as f32], color),
        TerrainVertex::new([(x + 1) as f32, (y + 1) as f32, z as f32], color),
        TerrainVertex::new([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32], color),
    ]);
}

pub fn face_front(x: usize, y: usize, z: usize, color: [u8; 4], light: u8) -> Vec<TerrainVertex> {
    let mut color = color;
    for i in 0..4 {
        color[i] = color[i] * ((light / 255) as f32) as u8;
    }

    return Vec::from([
        TerrainVertex::new([x as f32, y as f32, z as f32], color),
        TerrainVertex::new([(x + 1) as f32, y as f32, z as f32], color),
        TerrainVertex::new([x as f32, y as f32, (z + 1) as f32], color),
        TerrainVertex::new([(x + 1) as f32, y as f32, (z + 1) as f32], color),
    ]);
}

pub fn face_right(x: usize, y: usize, z: usize, color: [u8; 4], light: u8) -> Vec<TerrainVertex> {
    let mut color = color;
    for i in 0..4 {
        color[i] = color[i] * ((light / 255) as f32) as u8;
    }

    return Vec::from([
        TerrainVertex::new([(x + 1) as f32, y as f32, z as f32], color),
        TerrainVertex::new([(x + 1) as f32, (y + 1) as f32, z as f32], color),
        TerrainVertex::new([(x + 1) as f32, y as f32, (z + 1) as f32], color),
        TerrainVertex::new([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32], color),
    ]);
}

pub fn face_left(x: usize, y: usize, z: usize, color: [u8; 4], light: u8) -> Vec<TerrainVertex> {
    let mut color = color;
    for i in 0..4 {
        color[i] = color[i] * ((light / 255) as f32) as u8;
    }

    return Vec::from([
        TerrainVertex::new([x as f32, y as f32, z as f32], color),
        TerrainVertex::new([x as f32, y as f32, (z + 1) as f32], color),
        TerrainVertex::new([x as f32, (y + 1) as f32, z as f32], color),
        TerrainVertex::new([x as f32, (y + 1) as f32, (z + 1) as f32], color),
    ]);
}

