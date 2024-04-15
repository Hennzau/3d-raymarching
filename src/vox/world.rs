use crate::vox::chunk::ChunkData;

pub struct World {
    pub chunks: [[ChunkData; 2]; 2],
}

impl World {
    pub fn new() -> Self {
        return Self {
            chunks: [[ChunkData::new(), ChunkData::new()], [ChunkData::new(), ChunkData::new()]]
        };
    }
}