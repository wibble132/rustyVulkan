#![no_std]
// Unexpected arch "spirv"
#![allow(unexpected_cfgs)]

#[cfg(target_arch = "spirv")]
use spirv_std::glam;

#[repr(C)]
pub struct VertexData {
    pub position: glam::Vec2,
    pub colour: glam::Vec3,
}