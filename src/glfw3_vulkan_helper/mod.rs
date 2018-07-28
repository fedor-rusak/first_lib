use helper_old;

use std::ptr;
use std::ffi::{CString};


extern crate libc;

use self::libc::{c_int, c_float, c_void, c_uint, c_char};


pub mod glfw_types {
	#[allow(missing_copy_implementations)]
	pub enum GLFWmonitor {}

	#[allow(missing_copy_implementations)]
	pub enum GLFWwindow {}

	#[allow(missing_copy_implementations)]
	pub enum GLFWvkproc {}
}

use self::glfw_types::*;


pub mod vulkan_types;

use self::vulkan_types::*;


pub static GL_COLOR_BUFFER_BIT: c_uint = 0x00004000; //it is a macro constant :(

#[link(name = "glfw3")]
extern {
	fn glfwInit() -> c_int;
	fn glfwPollEvents() -> c_int;
	fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
	fn glfwMakeContextCurrent(window: *mut GLFWwindow) -> c_void;
	fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
	fn glfwSwapBuffers(window: *mut GLFWwindow) -> c_void;
	fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, onResizeCallback: extern fn(window: *mut GLFWwindow, i32, i32)) -> c_void;

	fn glfwVulkanSupported() -> c_int;
	fn glfwGetInstanceProcAddress(vkInstance: *mut VkInstance, function_name: *const c_char) -> *mut GLFWvkproc;
}

pub fn main() {
	println!("Hello from rust-ffi-glfw!");

	unsafe {
		let string = CString::new("Hello from rust-ffi-glfw!".as_bytes()).unwrap(); //tricky stuff. If written in one line string would vanish!
		let title = string.as_bytes_with_nul().as_ptr() as *const c_char;

		glfwInit();

		let check_result: c_int = glfwVulkanSupported();
		println!("Vulkan availability check result: {}", check_result);

		if check_result == 1 {
			println!("Vulkan loader is working!");

			let string = CString::new("vkCreateInstance".as_bytes()).unwrap(); //tricky stuff. If written in one line string would vanish!
			let function_name = string.as_bytes_with_nul().as_ptr() as *const c_char;

			let createInstanceProc = glfwGetInstanceProcAddress(ptr::null_mut(), function_name) as *const vkCreateInstance;
			// let instance: *mut VkInstance;
			(*createInstanceProc)(ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
		}
		else {
			println!("Vulkan loader not found!");
			return
		}
	}

	println!("Finished successfully!");
}