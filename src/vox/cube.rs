#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CubeData {
    pub color: [u8; 4],

    pub up_light: u8,
    pub down_light: u8,
    pub left_light: u8,
    pub right_light: u8,
    pub front_light: u8,
    pub back_light: u8,
}