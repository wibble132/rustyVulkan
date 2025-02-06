mod consts;
mod types;

pub use consts::*;
pub use types::*;
use std::ffi::{c_char, c_int};

#[derive(Debug)]
pub enum GLFWmonitor {}

#[derive(Debug)]
pub enum GLFWwindow {}

#[link(name = "glfw")]
extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwPollEvents();
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
}
