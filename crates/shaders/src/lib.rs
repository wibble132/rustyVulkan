#![no_std]

// Unexpected arch "spirv"
#![allow(unexpected_cfgs)]

use spirv_std::glam::{Vec2, Vec3, Vec4};
use spirv_std::spirv;

const POSITIONS: [Vec2; 3] = [
    Vec2::new(0.0, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
];
const COLOURS: [Vec3; 3] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
];

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_index: u32,
    #[spirv(position)] out_pos: &mut Vec4,
    frag_colour: &mut Vec3,
) {
    *out_pos = POSITIONS[vert_index as usize].extend(0.0).extend(1.0);
    *frag_colour = COLOURS[vert_index as usize];
}

#[spirv(fragment)]
pub fn main_fs(
    frag_colour: Vec3,
    output: &mut Vec4,
) {
    *output = frag_colour.extend(1.0);
}
