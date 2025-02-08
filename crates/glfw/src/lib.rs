#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! VK_MAKE_VERSION {
    ($major:expr,$minor:expr,$patch:expr) => {
        ((($major as u32) << 22u32) | (($minor as u32) << 12u32) | ($patch as u32))
    };
}

#[macro_export]
macro_rules! VK_MAKE_API_VERSION {
    ($variant:expr,$major:expr,$minor:expr,$patch:expr) => {
        ((($variant as u32) << 29u32)
            | (($major as u32) << 22u32)
            | (($minor as u32) << 12u32)
            | ($patch as u32))
    };
}

pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION!(0, 1, 0, 0);
