#![no_std]
// Unexpected arch "spirv"
#![allow(unexpected_cfgs)]

use shared::{UniformBufferObject, VertexData};
use spirv_std::glam::{Vec2, Vec3, Vec4};
use spirv_std::{spirv, Image};
use spirv_std::num_traits::Float;

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
    #[spirv(frag_coord)] point_coord: Vec4,
    output: &mut Vec4,
) {
    let x = point_coord.x as u32;
    let y = point_coord.y as u32;
    
    if x % 6 == 1 || y % 4 == 1 && (x + 2 * y) % 5 < 2 {
        *output = image.sample(*sampler, frag_tex_coord);
    } else {
        *output = frag_colour.extend(1.0);
    }
}
