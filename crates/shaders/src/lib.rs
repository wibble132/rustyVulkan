#![no_std]
// Unexpected arch "spirv"
#![allow(unexpected_cfgs)]

use shared::{UniformBufferObject, VertexData};
use spirv_std::glam::{Vec2, Vec3, Vec4};
use spirv_std::{spirv, Image};

#[spirv(vertex)]
pub fn main_vs(
    in_data: VertexData,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] ubo: &UniformBufferObject,
    #[spirv(position)] out_pos: &mut Vec4,
    out_frag_colour: &mut Vec3,
    out_frag_tex_coord: &mut Vec2,
) {
    let position = in_data.position.extend(0.0).extend(1.0);

    *out_pos = ubo.projection * ubo.view * ubo.model * position;
    *out_frag_colour = in_data.colour;
    *out_frag_tex_coord = in_data.tex_coord;
}

#[spirv(fragment)]
pub fn main_fs(
    frag_colour: Vec3,
    frag_tex_coord: Vec2,
    #[spirv(descriptor_set = 0, binding = 1)] image: &Image![2D, format = rgba8, sampled],
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &spirv_std::Sampler,
    output: &mut Vec4,
) {
    // Set output using sampler
    *output = image.sample(*sampler, frag_tex_coord);
}
