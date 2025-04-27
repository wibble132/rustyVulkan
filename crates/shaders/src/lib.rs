#![no_std]
// Unexpected arch "spirv"
#![allow(unexpected_cfgs)]

use shared::{UniformBufferObject, VertexData};
use spirv_std::glam::{Vec3, Vec4};
use spirv_std::spirv;

#[spirv(vertex)]
pub fn main_vs(
    in_data: VertexData,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] ubo: &UniformBufferObject,
    #[spirv(position)] out_pos: &mut Vec4,
    out_frag_colour: &mut Vec3,
) {
    let position = in_data.position.extend(0.0).extend(1.0);

    *out_pos = ubo.projection * ubo.view * ubo.model * position;
    *out_frag_colour = in_data.colour;
}

#[spirv(fragment)]
pub fn main_fs(frag_colour: Vec3, output: &mut Vec4) {
    *output = frag_colour.extend(1.0);
}
