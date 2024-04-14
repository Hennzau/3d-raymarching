use glam::{U16Vec4, UVec3};
use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::vox::renderer::terrain::TerrainVertex;

pub struct ChunkRenderData {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,

    pub index_count: usize,
}

impl ChunkRenderData {
    pub fn new(device: &Device) -> Self {
        let vertex_data: Vec<TerrainVertex> = Vec::from([
            TerrainVertex::new(UVec3::new(0, 0, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 0, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 1, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 1, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 0, 1), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 0, 1), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 1, 1), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 1, 1), U16Vec4::new(255, 0, 0, 1)),

            TerrainVertex::new(UVec3::new(0, 0, 3), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 0, 3), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 1, 3), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 1, 3), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 0, 4), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 0, 4), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 1, 4), U16Vec4::new(255, 255, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 1, 4), U16Vec4::new(255, 255, 0, 1)),

            TerrainVertex::new(UVec3::new(3, 0, 0), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(4, 0, 0), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(3, 1, 0), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(4, 1, 0), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(3, 0, 1), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(4, 0, 1), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(3, 1, 1), U16Vec4::new(255, 0, 255, 1)),
            TerrainVertex::new(UVec3::new(4, 1, 1), U16Vec4::new(255, 0, 255, 1)),

            TerrainVertex::new(UVec3::new(0, 3, 0), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(1, 3, 0), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(0, 4, 0), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(1, 4, 0), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(0, 3, 1), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(1, 3, 1), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(0, 4, 1), U16Vec4::new(255, 255, 255, 1)),
            TerrainVertex::new(UVec3::new(1, 4, 1), U16Vec4::new(255, 255, 255, 1)),

        ]);

        let mut index_data: Vec<u16> = Vec::from([
            0, 2, 1, 1, 2, 3, // Front
            4, 5, 6, 6, 5, 7, // Back
            0, 1, 4, 4, 1, 5, // Down
            3, 2, 6, 3, 6, 7, // Up
            0, 4, 2, 2, 4, 6, // Left
            1, 3, 5, 5, 3, 7, // Right
        ]);

        for i in 0..36 {
            index_data.push(index_data.get(i).unwrap() + 8)
        }

        for i in 0..36 {
            index_data.push(index_data.get(i).unwrap() + 16)
        }

        for i in 0..36 {
            index_data.push(index_data.get(i).unwrap() + 24)
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