mod consts;
mod types;

pub use consts::*;
pub use types::*;
use std::ffi::{c_char, c_int};

#[derive(Debug)]
pub enum Monitor {}

#[derive(Debug)]
pub enum Window {}

#[link(name = "glfw")]
extern "C" {
    pub fn init() -> c_int;
    pub fn terminate();
    pub fn windowHint(hint: c_int, value: c_int);
    pub fn createWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut Monitor,
        share: *mut Window,
    ) -> *mut Window;
    pub fn windowShouldClose(window: *mut Window) -> c_int;
    pub fn pollEvents();
    pub fn destroyWindow(window: *mut Window);
}
