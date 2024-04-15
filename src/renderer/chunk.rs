use glam::{
    U16Vec4,
    UVec3,
};

use wgpu::{
    Buffer,
    Device,
    util::DeviceExt,
};
use crate::renderer::cube::{face_down, face_back, face_front, face_indices, face_up, face_left, face_right};

use crate::renderer::terrain::TerrainVertex;
use crate::vox::chunk::{ChunkData, VoxelData};

pub struct ChunkRenderData {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,

    pub index_count: usize,
}

impl ChunkRenderData {
    pub fn new(device: &Device, chunk_x: usize, chunk_y: usize, chunk_z: usize, chunk: &ChunkData) -> Self {
        let mut vertex_data: Vec<TerrainVertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();

        let mut offset = 0;

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    match chunk.data[x][y][z] {
                        VoxelData::Air => {}
                        VoxelData::Plain(data) => {
                            let xx = chunk_x * 16 + x;
                            let yy = chunk_y * 16 + y;
                            let zz = chunk_z * 16 + z;

                            let mut vertices = face_front(xx, yy, zz, data.color, data.front_light);
                            let mut indices = face_indices(offset + 0);
                            vertex_data.append(&mut vertices);
                            index_data.append(&mut indices);

                            let mut vertices = face_back(xx, yy, zz, data.color, data.back_light);
                            let mut indices = face_indices(offset + 1);
                            vertex_data.append(&mut vertices);
                            index_data.append(&mut indices);

                            let mut vertices = face_down(xx, yy, zz, data.color, data.down_light);
                            let mut indices = face_indices(offset + 2);
                            vertex_data.append(&mut vertices);
                            index_data.append(&mut indices);

                            let mut vertices = face_up(xx, yy, zz, data.color, data.up_light);
                            let mut indices = face_indices(offset + 3);
                            vertex_data.append(&mut vertices);
                            index_data.append(&mut indices);

                            let mut vertices = face_left(xx, yy, zz, data.color, data.left_light);
                            let mut indices = face_indices(offset + 4);
                            vertex_data.append(&mut vertices);
                            index_data.append(&mut indices);

                            let mut vertices = face_right(xx, yy, zz, data.color, data.right_light);
                            let mut indices = face_indices(offset + 5);
                            vertex_data.append(&mut vertices);
                            index_data.append(&mut indices);

                            offset += 6;
                        }
                    }
                }
            }
        }

        let index_count = index_data.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            vertex_buffer,
            index_buffer,
            index_count,
        };
    }
}