use glfw3_helper::*;
use glfw3_helper::glfw_types::*;

use std::mem;
use std::ptr;
use std::ffi::{CString};


extern crate libc;

use self::libc::c_char;


#[macro_use]
pub mod vulkan_types;

use self::vulkan_types::*;


// #[cfg(all(windows))]
// fn extension_names() -> Vec<*const i8> {
//     vec![
//         Surface::name().as_ptr(),
//         Win32Surface::name().as_ptr(),
//         DebugReport::name().as_ptr(),
//     ]
// }

///
/// This is the most complex thing in whole library by far.
///
/// # Idea
///
/// This piece of code should render something simple on screen using API from GLFW3 and Vulkan.
///
/// # Implementation
///
/// Corresponding docs are [Glfw3](http://www.glfw.org/docs/latest/vulkan_guide.html) and [Vulkan API Tutorial](https://vulkan.lunarg.com/doc/sdk/1.0.57.0/windows/tutorial/html/index.html)
///
/// 1) GLFW3 initialized.
/// 2) Vulkan support is checked (technically speaking it looks for loader library)
///
/// 3) VkInstance is created (thing that is responsible for stateful communication with Vulkan-compatibe devices)
/// 4) vkEnumeratePhysicalDevices is called two times in order to get data about first Vulkan-compatible physical device
///
/// 5) vkDestroyInstance is called to clean up everything
///
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


		let check_result: i32 = glfwVulkanSupported();

		if check_result == GLFW_TRUE {
			println!("Vulkan loader is working!");

			let string = CString::new("vkCreateInstance").unwrap(); //tricky stuff. If written in one line string would vanish!
			let function_name = string.as_ptr() as *const c_char;


            //VkInstance part START

            //this is some black magic thing
			let create_instance_proc = glfwGetInstanceProcAddress(ptr::null_mut(), function_name);
            let create_instance_function: vkCreateInstance = mem::transmute(create_instance_proc);


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
                flags: VkFlags::Empty,
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
                println!("Failed to create instance! {:?}", instance_creation_result);
                return 1
            };

            //VkInstance part END


            //vkEnumeratePhysicalDevices part START

            let string = CString::new("vkEnumeratePhysicalDevices").unwrap(); //tricky stuff. If written in one line string would vanish!
            let function_name = string.as_ptr() as *const c_char;

            //this is some black magic thing
            let enumerate_devices_proc = glfwGetInstanceProcAddress(ptr::null_mut(), function_name);
            let enumerate_devices_function: vkEnumeratePhysicalDevices = mem::transmute(enumerate_devices_proc);

            let mut device_count = mem::uninitialized();
            (enumerate_devices_function)(instance, &mut device_count, ptr::null_mut());

            println!("Vulkan physical device count: {}", device_count);

            let mut physical_devices = Vec::<VkPhysicalDevice>::with_capacity(device_count as usize);
            let physical_device_enumarate_result = 
                (enumerate_devices_function)(instance, &mut device_count, physical_devices.as_mut_ptr());

            if physical_device_enumarate_result == VkResult::Success {
                println!("Successfully enumerated physical devices!");
            }
            else {
                println!("Failed to enumerate physical devices!");
                return -1
            }

            //vkEnumeratePhysicalDevices part END



            // let string = CString::new("vkCreateDevice").unwrap(); //tricky stuff. If written in one line string would vanish!
            // let function_name = string.as_ptr() as *const c_char;

            // //this is some black magic thing
            // let create_device_proc = glfwGetInstanceProcAddress(&mut instance, function_name);
            // let _create_device_function: vkCreateDevice = mem::transmute(create_device_proc);


            //vkDestroyInstance START

            let string = CString::new("vkDestroyInstance").unwrap(); //tricky stuff. If written in one line string would vanish!
            let function_name = string.as_ptr() as *const c_char;

            //this is some black magic thing
            let destroy_instance_proc = glfwGetInstanceProcAddress(ptr::null_mut(), function_name);
            let destroy_instance_function: vkDestroyInstance = mem::transmute(destroy_instance_proc);

            (destroy_instance_function)(instance, ptr::null());

            println!("Instance was destroyed successfully!");

            //vkDestroyInstance END
		}
		else {
			println!("Vulkan loader not found!");
			return 1
		}
	}


	println!("Finished successfully!");
    return 0
}