use std::mem;
use std::ptr;
use std::ffi::{CString};


use glfw3_helper::*;
use glfw3_helper::glfw_types::*;


#[macro_use]
pub mod vulkan_types;

use self::vulkan_types::*;

mod vk_functions {
    pub static CREATE_INSTANCE: &'static str = "vkCreateInstance";
    pub static ENUMERATE_PHYSICAL_DEVICES: &'static str = "vkEnumeratePhysicalDevices";
    pub static GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES: &'static str = "vkGetPhysicalDeviceQueueFamilyProperties";
    pub static DESTROY_INSTANCE: &'static str = "vkDestroyInstance";
}


///
/// This function creates Vulkan instance. Which means:
///
/// 1) create application info
/// 2) create "instance create info"
/// 3) get pointer to Vulkan function
/// 4) call instance creation
///
unsafe fn call_create_instance(result_instance: &mut VkInstance) -> VkResult {
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

    let create_info = VkInstanceCreateInfo {
        s_type: VkStructureType::InstanceCreateInfo,
        p_next: ptr::null(),
        flags: flags::EMPTY,
        p_application_info: &appinfo,
        pp_enabled_layer_names: ptr::null(),
        enabled_layer_count: 0 as u32,
        pp_enabled_extension_names: ptr::null(),
        enabled_extension_count: 0 as u32,
    };

    let create_instance_function: vkCreateInstance =
        get_vk_function_with_null_vk_instance(vk_functions::CREATE_INSTANCE);

    let instance_creation_result = create_instance_function(&create_info, ptr::null(), result_instance);

    instance_creation_result
}

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
/// 5) vkGetPhysicalDeviceQueueFamilyProperties to find a queueFamily that can be used for rendering
///
/// 6) vkDestroyInstance is called to clean up everything
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


            //vkCreateInstance START

            let mut instance: VkInstance = mem::uninitialized();

			let instance_creation_result = call_create_instance(&mut instance);

            if instance_creation_result == VkResult::Success {
                println!("Instance was created successfully!");
            }
            else {
                println!("Failed to create instance! {:?}", instance_creation_result);
                return 1
            };

            //vkCreateInstance END


            //vkEnumeratePhysicalDevices START

            let enumerate_devices_function: vkEnumeratePhysicalDevices =
                get_vk_function_with_null_vk_instance(vk_functions::ENUMERATE_PHYSICAL_DEVICES);

            let mut device_count = mem::uninitialized();
            enumerate_devices_function(instance, &mut device_count, ptr::null_mut());

            println!("Vulkan physical device count: {}", device_count);

            let mut physical_devices = Vec::<VkPhysicalDevice>::with_capacity(device_count as usize);

            let physical_device_enumarate_result = 
                enumerate_devices_function(instance, &mut device_count, physical_devices.as_mut_ptr());

            if physical_device_enumarate_result == VkResult::Success {
                println!("Successfully enumerated physical devices!");
            }
            else {
                println!("Failed to enumerate physical devices!");
                return -1
            }

            //driver does not know about our internal counter in Vec
            physical_devices.set_len(device_count as usize);
            let chosen_physical_device = physical_devices[0];

            //vkEnumeratePhysicalDevices END


            //vkGetPhysicalDeviceQueueFamilyProperties START

            let get_queue_family_properties_function: vkGetPhysicalDeviceQueueFamilyProperties =
                get_vk_function_with_null_vk_instance(vk_functions::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES);

            let mut queue_family_count = mem::uninitialized();
            get_queue_family_properties_function(chosen_physical_device, &mut queue_family_count, ptr::null_mut());

            println!("On first physical device we have this queue family count: {}", queue_family_count);

            let mut queue_families_properties = Vec::<VkQueueFamilyProperties>::with_capacity(queue_family_count as usize);

            get_queue_family_properties_function(chosen_physical_device, &mut queue_family_count, queue_families_properties.as_mut_ptr());

            //driver does not know about our internal counter in Vec
            queue_families_properties.set_len(queue_family_count as usize);

            for i in 0..queue_families_properties.len() {
                println!("Queue family number {} has flags = {:b} and queue count = {}",
                    i, queue_families_properties[i].queue_flags, queue_families_properties[i].queue_count);

            }

            //vkGetPhysicalDeviceQueueFamilyProperties END


            //vkDestroyInstance START

            let destroy_instance_function: vkDestroyInstance = 
                get_vk_function_with_null_vk_instance(vk_functions::DESTROY_INSTANCE);

            destroy_instance_function(instance, ptr::null());

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