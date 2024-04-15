use crate::vox::cube::CubeData;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum VoxelData {
    Air,
    Plain(CubeData),
}

pub struct ChunkData {
    pub data: [[[VoxelData; 16]; 16]; 16],
}

impl ChunkData {
    pub fn new() -> Self {
        let mut data = [[[VoxelData::Air; 16]; 16]; 16];

        for x in 0..16 {
            for y in 0..3 {
                for z in 0..3 {
                    data[x][y][2 * z] = VoxelData::Plain(CubeData {
                        color: [255, 255, 255, 255],

                        up_light: 255,
                        down_light: 255,
                        left_light: 255,
                        right_light: 255,
                        front_light: 255,
                        back_light: 255,
                    });
                }
            }
        }

        return Self {
            data
        };
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &VoxelData {
        return &self.data[x][y][z];
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, cube: VoxelData) {
        self.data[x][y][z] = cube;
    }
}