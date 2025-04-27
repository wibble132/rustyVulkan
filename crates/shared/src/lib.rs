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

#[repr(C)]
pub struct UniformBufferObject {
    pub model: glam::Mat4,
    pub view: glam::Mat4,
    pub projection: glam::Mat4,
}
