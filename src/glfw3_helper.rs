#![allow(non_camel_case_types)]

extern crate libc;

use std::mem;
use std::ptr;
use std::ffi::{CString};

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

type uint32_t = u32;

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
	pub fn glfwGetRequiredInstanceExtensions(count : *mut uint32_t) -> *const *const c_char;
}

pub const GL_COLOR_BUFFER_BIT: c_uint = 0x00004000; //it is a macro constant :(

///
/// This function hides some pointer juggling for retrieving Vulkan-specific functions from GLFW3.
///
pub unsafe fn get_vk_function<T>(vk_instance: *mut VkInstance, function_name: &str) -> T {
    let string = CString::new(function_name).unwrap(); //tricky stuff. If written in one line string would vanish!
    let function_name = string.as_ptr() as *const c_char;


    //this is some black magic thing
    let proc = glfwGetInstanceProcAddress(vk_instance, function_name);

    //this part is deepest depth of black magic
    let result_function = mem::transmute_copy::<GLFWvkproc, T>(&proc);
    mem::forget(proc);

    result_function
}

///
/// Hellper for calling get_vk_function with default first parameter.
///
pub unsafe fn get_vk_function_with_null_vk_instance<T>( function_name: &str) -> T {
    get_vk_function::<T>(ptr::null_mut(), function_name)
}