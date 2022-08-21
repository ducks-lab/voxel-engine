
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub color: [f32; 4],
    pub is_empty: bool,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkUniform {
    pub position: [i32; 3],
    pub voxels: Vec<Voxel>,
}

impl ChunkUniform {
    pub fn new(position: [i32; 3], voxels: Vec<Voxel>) -> Self {
        Self {
            position,
            voxels,
        }
    }
}