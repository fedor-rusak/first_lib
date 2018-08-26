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
    pub static CREATE_LOGICAL_DEVICE: &'static str = "vkCreateDevice";
    pub static CREATE_COMMAND_POOL: &'static str = "vkCreateCommandPool";
    pub static ALLOCATE_COMMAND_BUFFERS: &'static str = "vkAllocateCommandBuffers";
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

    let application_info = VkApplicationInfo {
        p_application_name: raw_name,
        s_type: VkStructureType::ApplicationInfo,
        p_next: ptr::null(),
        application_version: 0,
        p_engine_name: raw_name,
        engine_version: 0,
        api_version: vk_make_version!(1, 1, 77),
    };

    let instance_create_info = VkInstanceCreateInfo {
        s_type: VkStructureType::InstanceCreateInfo,
        p_next: ptr::null(),
        flags: flags::EMPTY,
        p_application_info: &application_info,
        pp_enabled_layer_names: ptr::null(),
        enabled_layer_count: 0 as u32,
        pp_enabled_extension_names: ptr::null(),
        enabled_extension_count: 0 as u32,
    };

    let create_instance_function: vkCreateInstance =
        get_vk_function_with_null_vk_instance(vk_functions::CREATE_INSTANCE);

    let mut result_instance: VkInstance = mem::uninitialized();

    let instance_creation_result = create_instance_function(&instance_create_info, ptr::null(), &mut result_instance);

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

unsafe fn call_create_logical_device(physical_device: VkPhysicalDevice, chosen_queue_family_index: u32) -> Result<VkDevice, VkResult> {
    let priorities = [0.0];

    let device_queue_create_info = DeviceQueueCreateInfo {
        s_type: VkStructureType::DeviceQueueCreateInfo,
        p_next: ptr::null(),
        flags: 0,
        queue_family_index: chosen_queue_family_index,
        queue_count: 1,
        p_queue_priorities: priorities.as_ptr()
    };

    let device_create_info = DeviceCreateInfo {
        s_type: VkStructureType::DeviceCreateInfo,
        p_next: ptr::null(),
        flags: 0,
        queue_create_info_count: 1,
        p_queue_create_infos: &device_queue_create_info,
        enabled_layer_count: 0,
        pp_enabled_layer_names: ptr::null(),
        enabled_extension_count: 0,
        pp_enabled_extension_names: ptr::null(),
        p_enabled_features: ptr::null()
    };


    let create_logical_device_function: vkCreateDevice = 
        get_vk_function_with_null_vk_instance(vk_functions::CREATE_LOGICAL_DEVICE);

    let mut logical_device: VkDevice = mem::uninitialized();

    let logical_device_creation_result =
        create_logical_device_function(physical_device, &device_create_info, ptr::null_mut(), &mut logical_device);

    if logical_device_creation_result == VkResult::Success {
        Ok(logical_device)
    }
    else {
        Err(logical_device_creation_result)
    }
}

fn bit_and(value: u32, bit_mask_value: u32) -> bool {
    value & bit_mask_value == bit_mask_value
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
/// 5) vkGetPhysicalDeviceQueueFamilyProperties to find a queueFamily that can be used for rendering graphics
/// 6) vkCreateDevice to create a logical device with chosen queue family that will be used later for sending commands
///
/// 7) vkDestroyInstance is called to clean up everything
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

            let mut chosen_queue_family_index = 0;
            let mut index_was_found = false;
            for i in 0..queue_families_properties.len() {
                println!("  Queue family number {} has flags = {:b} and queue count = {}",
                    i, queue_families_properties[i].queue_flags, queue_families_properties[i].queue_count);

                if bit_and(queue_families_properties[i].queue_flags, flags::QUEUE_GRAPHICS_BIT)  {
                    println!("Queue family with graphics support (bitmask {:b}) was found at index: {}", flags::QUEUE_GRAPHICS_BIT, i);
                    chosen_queue_family_index = i as u32;
                    index_was_found = true;

                    break;
                }
            }

            assert_eq!(index_was_found, true, "Index with graphics support must be found!");

            //vkGetPhysicalDeviceQueueFamilyProperties END


            //vkCreateDevice START

            let logical_device = call_create_logical_device(chosen_physical_device, chosen_queue_family_index).expect("Logical device must be created successfully!");

            //vkCreateDevice END


            //vkCreateCommandPool START

            let cmd_pool_create_info = VkCommandPoolCreateInfo{
                s_type: VkStructureType::CommandPoolCreateInfo,
                p_next: ptr::null(),
                flags: 0,
                queue_family_index: chosen_queue_family_index
            };

            let create_command_pool_function: vkCreateCommandPool = 
                get_vk_function_with_null_vk_instance(vk_functions::CREATE_COMMAND_POOL);

            let mut command_pool: VkCommandPool = mem::uninitialized();

            let command_pool_creation_result = create_command_pool_function(logical_device, &cmd_pool_create_info, ptr::null(), &mut command_pool);

            if command_pool_creation_result == VkResult::Success {
                println!("Command pool was created successfully!");
            }
            else {
                println!("Failed to create command pool!");
                return -1
            }

            //vkCreateCommandPool END


            //vkCreateCommandPool START

            let buffer_allocation_info = VkCommandBufferAllocateInfo {
                s_type: VkStructureType::CommandBufferAllocateInfo,
                p_next: ptr::null(),
                command_pool: command_pool,
                level: VkCommandBufferLevel::Primary,
                command_buffer_count: 1
            };

            let allocate_command_buffers_function: vkAllocateCommandBuffers = 
                get_vk_function_with_null_vk_instance(vk_functions::ALLOCATE_COMMAND_BUFFERS);

            let mut command_buffers: VkCommandBuffer = mem::uninitialized();

            let command_buffers_allocation_result = allocate_command_buffers_function(logical_device, &buffer_allocation_info, &mut command_buffers);

            if command_buffers_allocation_result == VkResult::Success {
                println!("Command buffer was allocated successfully!");
            }
            else {
                println!("Failed to allocate command buffer!");
                return -1
            }

            //vkCreateCommandPool END


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