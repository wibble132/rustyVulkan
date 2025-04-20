#![no_std]
// Unexpected arch "spirv"
#![allow(unexpected_cfgs)]

use spirv_std::glam::{Vec2, Vec3, Vec4};
use spirv_std::spirv;

#[spirv(vertex)]
pub fn main_vs(
    in_position: Vec2,
    in_color: Vec3,
    #[spirv(position)] out_pos: &mut Vec4,
    frag_colour: &mut Vec3,
) {
    *out_pos = in_position.extend(0.0).extend(1.0);
    *frag_colour = in_color;
}

#[spirv(fragment)]
pub fn main_fs(frag_colour: Vec3, output: &mut Vec4) {
    *output = frag_colour.extend(1.0);
}
