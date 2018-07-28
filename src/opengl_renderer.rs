use glfw3_helper::*;
use glfw3_helper::glfw_types::*;

use helper_old;

use std::ptr;
use std::ffi::{CString};

extern crate libc;

use self::libc::{c_int, c_float, c_void, c_uint, c_char};

#[link(name = "glew32")]
extern "stdcall" { //this is some wicked mumbo-jumbo for windows macro and dll
	fn glewInit() -> c_int;
}

#[link(name = "OpenGL32")]
extern "stdcall" {
	fn glClearColor(r: c_float, g: c_float, b: c_float, a: c_float) -> c_void;
	fn glClear(bitmask: c_uint) -> c_void;
}


#[allow(unused_variables)]
extern fn on_resize_callback(window: *mut GLFWwindow, width: i32, height: i32) {
    println!("I'm called from C with value {0} and {1}", width, height);
}


pub fn main() -> i32 {
	println!("Hello from rust-ffi-glfw!");

	unsafe {
		let init_result = glfwInit();

        if init_result == GLFW_TRUE {
            println!("GLFW3 initialized!");
        }
        else {
            println!("GLFW3 init failed!");
            return 1
        }


		let string = CString::new("Hello from rust-ffi-glfw!").unwrap(); //tricky stuff. If written in one line string would vanish!
		let title = string.as_ptr() as *const c_char;

		let window = glfwCreateWindow(800 as c_int, 600 as c_int, title, ptr::null_mut(), ptr::null_mut());


		glfwSetWindowSizeCallback(window, on_resize_callback);


		glfwMakeContextCurrent(window);
		println!("GLFW window was opened!");


		if glewInit() == 0 {
			println!("GLEW initialized!");
		}
		else {
			println!("GLEW failed to initialize!");
			return 1
		}

		glClearColor(0.3, 0.4, 0.1, 1.0);

		loop {
			glfwPollEvents();

			if glfwWindowShouldClose(window) == 1 {
				break;
			}

			glClear(GL_COLOR_BUFFER_BIT);

			glfwSwapBuffers(window);
		};
	}

	println!("GLFW window was closed successfully!");

	println!("Answer for evrything is {}!", helper_old::answer_for_everything());
	return 0
}