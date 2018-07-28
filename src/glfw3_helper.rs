extern crate libc;

use self::libc::{c_int, c_void, c_uint, c_char};


use vulkan_renderer::vulkan_types::VkInstance;

pub mod glfw_types {
#![allow(dead_code)]
    extern crate libc;

	#[allow(missing_copy_implementations)]
	pub enum GLFWmonitor {}

	#[allow(missing_copy_implementations)]
	pub enum GLFWwindow {}

	pub type GLFWvkproc = *const libc::c_void;

    pub const GLFW_TRUE: i32 = 1;
    pub const GLFW_FALSE: i32 = 0;
}

use self::glfw_types::*;

#[link(name = "glfw3")]
extern {
	pub fn glfwInit() -> c_int;

	pub fn glfwPollEvents() -> c_int;
	pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
	pub fn glfwMakeContextCurrent(window: *mut GLFWwindow) -> c_void;
	pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
	pub fn glfwSwapBuffers(window: *mut GLFWwindow) -> c_void;
	pub fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, onResizeCallback: extern fn(window: *mut GLFWwindow, i32, i32)) -> c_void;

	pub fn glfwVulkanSupported() -> c_int;
	pub fn glfwGetInstanceProcAddress(vkInstance: *mut VkInstance, function_name: *const c_char) -> GLFWvkproc;
}

pub const GL_COLOR_BUFFER_BIT: c_uint = 0x00004000; //it is a macro constant :(