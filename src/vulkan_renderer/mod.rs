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
unsafe fn call_create_instance() -> Result<VkInstance, VkResult> {
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

    let mut result_instance: VkInstance = mem::uninitialized();

    let instance_creation_result = create_instance_function(&create_info, ptr::null(), &mut result_instance);

    if instance_creation_result == VkResult::Success {
        Ok(result_instance)
    }
    else {
        Err(instance_creation_result)
    }
}

unsafe fn get_physical_devices(instance: VkInstance) -> Result<Vec<VkPhysicalDevice>, VkResult> {
    let enumerate_devices_function: vkEnumeratePhysicalDevices =
        get_vk_function_with_null_vk_instance(vk_functions::ENUMERATE_PHYSICAL_DEVICES);

    let mut device_count = mem::uninitialized();
    enumerate_devices_function(instance, &mut device_count, ptr::null_mut());

    let mut physical_devices = Vec::<VkPhysicalDevice>::with_capacity(device_count as usize);

    let physical_device_enumarate_result = 
        enumerate_devices_function(instance, &mut device_count, physical_devices.as_mut_ptr());

    if physical_device_enumarate_result == VkResult::Success {
        //driver does not know about our internal counter in Vec
        physical_devices.set_len(device_count as usize);
        
        Ok(physical_devices)
    }
    else {
        Err(physical_device_enumarate_result)
    }
}

unsafe fn get_queue_family_properties(physical_device: VkPhysicalDevice) -> Result<Vec<VkQueueFamilyProperties>, &'static str> {
    let get_queue_family_properties_function: vkGetPhysicalDeviceQueueFamilyProperties =
        get_vk_function_with_null_vk_instance(vk_functions::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES);

    let mut queue_family_count = mem::uninitialized();
    get_queue_family_properties_function(physical_device, &mut queue_family_count, ptr::null_mut());

    let mut queue_families_properties = Vec::<VkQueueFamilyProperties>::with_capacity(queue_family_count as usize);

    get_queue_family_properties_function(physical_device, &mut queue_family_count, queue_families_properties.as_mut_ptr());

    if queue_family_count > 0 {
        //driver does not know about our internal counter in Vec
        queue_families_properties.set_len(queue_family_count as usize);

        Ok(queue_families_properties)
    }
    else {
        Err("No queue family properties were found!")
    }
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
    println!();
	println!("Hello from render_lib Vulkan renderer!");

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

			let vk_instance = call_create_instance().expect("To create Vulkan instance successfully!");

            //vkCreateInstance END


            //vkEnumeratePhysicalDevices START

            let physical_devices = get_physical_devices(vk_instance).expect("Successfully enumerated physical devices!");

            println!("Vulkan physical device count: {}", physical_devices.len());

            let chosen_physical_device = physical_devices[0];

            //vkEnumeratePhysicalDevices END


            //vkGetPhysicalDeviceQueueFamilyProperties START

            let queue_families_properties =
                get_queue_family_properties(chosen_physical_device).expect("Non-zero number of queue family properties for chosen physical device!");

            println!("Chosen Vulkan physical device (bear with me) queue family properties count: {}", physical_devices.len());

            for i in 0..queue_families_properties.len() {
                println!("  Queue family number {} has flags = {:b} and queue count = {}",
                    i, queue_families_properties[i].queue_flags, queue_families_properties[i].queue_count);
            }

            //vkGetPhysicalDeviceQueueFamilyProperties END


            //vkDestroyInstance START

            let destroy_instance_function: vkDestroyInstance = 
                get_vk_function_with_null_vk_instance(vk_functions::DESTROY_INSTANCE);

            destroy_instance_function(vk_instance, ptr::null());

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