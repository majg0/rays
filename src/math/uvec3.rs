#[derive(Clone, Copy, Debug)]
pub struct UVec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl UVec3 {
    pub fn new(x: u32, y: u32, z: u32) -> UVec3 {
        UVec3 { x, y, z }
    }
}
