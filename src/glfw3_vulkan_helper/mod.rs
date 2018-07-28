use std::mem;
use std::ptr;
use std::ffi::{CString};


extern crate libc;

use self::libc::{c_int, c_uint, c_char};


pub mod glfw_types {
    extern crate libc;

	#[allow(missing_copy_implementations)]
	pub enum GLFWmonitor {}

	#[allow(missing_copy_implementations)]
	pub enum GLFWwindow {}

	pub type GLFWvkproc = *const libc::c_void;
}

use self::glfw_types::*;


#[macro_use]
pub mod vulkan_types;

use self::vulkan_types::*;


pub static GL_COLOR_BUFFER_BIT: c_uint = 0x00004000; //it is a macro constant :(

#[link(name = "glfw3")]
extern {
	fn glfwInit() -> c_int;

	fn glfwVulkanSupported() -> c_int;
	fn glfwGetInstanceProcAddress(vkInstance: *mut VkInstance, function_name: *const c_char) -> GLFWvkproc;
}


// #[cfg(all(windows))]
// fn extension_names() -> Vec<*const i8> {
//     vec![
//         Surface::name().as_ptr(),
//         Win32Surface::name().as_ptr(),
//         DebugReport::name().as_ptr(),
//     ]
// }

pub fn main() -> i32 {
	println!("Hello from rust-ffi-glfw!");

	unsafe {
		glfwInit();

		let check_result: c_int = glfwVulkanSupported();
		println!("Vulkan availability check result: {}", check_result);

		if check_result == 1 {
			println!("Vulkan loader is working!");

			let string = CString::new("vkCreateInstance").unwrap(); //tricky stuff. If written in one line string would vanish!
			let function_name = string.as_ptr() as *const c_char;

            //this is some black magic thing
			let create_instance_proc = glfwGetInstanceProcAddress(ptr::null_mut(), function_name);
            let create_instance_function:  vkCreateInstance = mem::transmute(create_instance_proc);


            let app_name = CString::new("VulkanTest").unwrap();
            let raw_name = app_name.as_ptr();

            let appinfo = VkApplicationInfo {
                p_application_name: raw_name,
                s_type: VkStructureType::ApplicationInfo,
                p_next: ptr::null(),
                application_version: 0,
                p_engine_name: raw_name,
                engine_version: 0,
                api_version: vk_make_version!(1, 1, 77),
            };

            // let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
            // let layers_names_raw: Vec<*const i8> = layer_names
            //     .iter()
            //     .map(|raw_layer_name| raw_layer_name.as_ptr())
            //     .collect();

            // let extension_names_raw = extension_names();

            let create_info = VkInstanceCreateInfo {
                s_type: VkStructureType::InstanceCreateInfo,
                p_next: ptr::null(),
                flags: Flags::Empty,
                p_application_info: &appinfo,
                pp_enabled_layer_names: ptr::null(),
                enabled_layer_count: 0 as u32,
                pp_enabled_extension_names: ptr::null(),
                enabled_extension_count: 0 as u32,
            };

            let mut instance: VkInstance = mem::uninitialized();

			let instance_creation_result = (create_instance_function)(&create_info, ptr::null(), &mut instance);

            if instance_creation_result == VkResult::Success {
                println!("Instance was created successfully!");
            }
            else {
                println!("Failed to create instance!");
                return 1
            };
		}
		else {
			println!("Vulkan loader not found!");
			return 1
		}
	}

	println!("Finished successfully!");
    return 0
}