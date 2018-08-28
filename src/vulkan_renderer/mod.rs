use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::slice;


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
    pub static DESTROY_SURFACE: &'static str = "vkDestroySurfaceKHR";
    pub static DESTROY_INSTANCE: &'static str = "vkDestroyInstance";
}


unsafe fn string_from_c_str(c_str: *const c_char) -> &'static str {
    let c_str = CStr::from_ptr(c_str);
    c_str.to_str().unwrap()
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


    let mut extension_count: u32 = mem::uninitialized();
    let extension_names = glfwGetRequiredInstanceExtensions(&mut extension_count);
    let data_slice = slice::from_raw_parts(extension_names, extension_count as usize);

    let mut glfw3_advised_extensions = Vec::new();
    for i in 0..extension_count as usize {
        glfw3_advised_extensions.push(string_from_c_str(data_slice[i]));
    }

    println!("  GLFW3 advised to use {} extension(s): {:?}!", extension_count, glfw3_advised_extensions);

    let instance_create_info = VkInstanceCreateInfo {
        s_type: VkStructureType::InstanceCreateInfo,
        p_next: ptr::null(),
        flags: flags::EMPTY,
        p_application_info: &application_info,
        pp_enabled_layer_names: ptr::null(),
        enabled_layer_count: 0 as u32,
        pp_enabled_extension_names: extension_names,
        enabled_extension_count: extension_count
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

    let really_important_extension_name = CString::new(VK_KHR_SWAPCHAIN_EXTENSION_NAME).unwrap();

    let extension_names_raw = [really_important_extension_name.as_ptr()];

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
        enabled_extension_count: extension_names_raw.len() as u32,
        pp_enabled_extension_names: extension_names_raw.as_ptr(),
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
/// 6) glfwGetPhysicalDevicePresentationSupport to check if our queue family supports presentation (means we can show images via swapchain surfaces)
/// 7) vkCreateDevice to create a logical device with chosen queue family that will be used later for sending commands
///
/// 8) vkCreateCommandPool Alice is falling deeper...
/// 9) vkCommandBufferAllocate and deeper...
/// 10) create surface!!!!! There is even a window shown for a second!!! This is WSI in action and native stuff is kep inside GLFW3!!!
///
/// 11) vkDestroySurfaceKHR to free WSI related stuff
/// 12) vkDestroyInstance is called to clean up everything
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


            //glfwGetPhysicalDevicePresentationSupport START

            let presentation_check_result =
                glfwGetPhysicalDevicePresentationSupport(vk_instance, chosen_physical_device, chosen_queue_family_index);

            if GLFW_TRUE == presentation_check_result {
                println!("Chosen queue family index has presentation support!");
            }
            else {
                println!("You need a separate family queue index for presentation! Currently not supported!");
                return -1
            }

            //glfwGetPhysicalDevicePresentationSupport END


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
                println!("Failed to create command pool! Result: {:?}", command_pool_creation_result);
                return -1
            }

            //vkCreateCommandPool END


            //vkCommandBufferAllocate START

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
                println!("Failed to allocate command buffer! Result: {:?}", command_buffers_allocation_result);
                return -1
            }

            //vkCommandBufferAllocate END


            //VkSurfaceKHR creation START

            glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);

            let string = CString::new("Hello from render_lib Vulkan renderer!").unwrap(); //tricky stuff. If written in one line string would vanish!
            let title = string.as_ptr() as *const c_char;
            let window: *mut GLFWwindow = glfwCreateWindow(640, 480, title, ptr::null_mut(), ptr::null_mut());

            let mut surface: VkSurfaceKHR = mem::uninitialized();
            let surface_creation_result = glfwCreateWindowSurface(vk_instance, window, ptr::null(), &mut surface);

            if surface_creation_result == VkResult::Success {
                println!("Surface was created with help of GLFW3!");
            }
            else {
                println!("Failed to create surface with help of GLFW3! Result: {:?}", surface_creation_result);
            }
            
            //VkSurfaceKHR creation END


            //swapchain stuff START

                // Get the list of VkFormats that are supported:
                // uint32_t formatCount;
                // res = vkGetPhysicalDeviceSurfaceFormatsKHR(info.gpus[0], info.surface, &formatCount, NULL);
                // assert(res == VK_SUCCESS);
                // VkSurfaceFormatKHR *surfFormats = (VkSurfaceFormatKHR *)malloc(formatCount * sizeof(VkSurfaceFormatKHR));
                // res = vkGetPhysicalDeviceSurfaceFormatsKHR(info.gpus[0], info.surface, &formatCount, surfFormats);
                // assert(res == VK_SUCCESS);
                // // If the format list includes just one entry of VK_FORMAT_UNDEFINED,
                // // the surface has no preferred format.  Otherwise, at least one
                // // supported format will be returned.
                // if (formatCount == 1 && surfFormats[0].format == VK_FORMAT_UNDEFINED) {
                //     info.format = VK_FORMAT_B8G8R8A8_UNORM;
                // } else {
                //     assert(formatCount >= 1);
                //     info.format = surfFormats[0].format;
                // }
                // free(surfFormats);


                // VkSurfaceCapabilitiesKHR surfCapabilities;

                // res = vkGetPhysicalDeviceSurfaceCapabilitiesKHR(info.gpus[0], info.surface, &surfCapabilities);
                // assert(res == VK_SUCCESS);

                // VkExtent2D swapchainExtent;
                // // width and height are either both 0xFFFFFFFF, or both not 0xFFFFFFFF.
                // if (surfCapabilities.currentExtent.width == 0xFFFFFFFF) {
                //     // If the surface size is undefined, the size is set to
                //     // the size of the images requested.
                //     swapchainExtent.width = info.width;
                //     swapchainExtent.height = info.height;
                //     if (swapchainExtent.width < surfCapabilities.minImageExtent.width) {
                //         swapchainExtent.width = surfCapabilities.minImageExtent.width;
                //     } else if (swapchainExtent.width > surfCapabilities.maxImageExtent.width) {
                //         swapchainExtent.width = surfCapabilities.maxImageExtent.width;
                //     }

                //     if (swapchainExtent.height < surfCapabilities.minImageExtent.height) {
                //         swapchainExtent.height = surfCapabilities.minImageExtent.height;
                //     } else if (swapchainExtent.height > surfCapabilities.maxImageExtent.height) {
                //         swapchainExtent.height = surfCapabilities.maxImageExtent.height;
                //     }
                // } else {
                //     // If the surface size is defined, the swap chain size must match
                //     swapchainExtent = surfCapabilities.currentExtent;
                // }

                // // The FIFO present mode is guaranteed by the spec to be supported
                // // Also note that current Android driver only supports FIFO

                // // Determine the number of VkImage's to use in the swap chain.
                // // We need to acquire only 1 presentable image at at time.
                // // Asking for minImageCount images ensures that we can acquire
                // // 1 presentable image as long as we present it before attempting
                // // to acquire another.
                // uint32_t desiredNumberOfSwapChainImages = surfCapabilities.minImageCount;


                // VkSurfaceTransformFlagBitsKHR preTransform;
                // if (surfCapabilities.supportedTransforms & VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR) {
                //     preTransform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
                // } else {
                //     preTransform = surfCapabilities.currentTransform;
                // }


                // VkPresentModeKHR swapchainPresentMode = VK_PRESENT_MODE_FIFO_KHR;


                // // Find a supported composite alpha mode - one of these is guaranteed to be set
                //     VkCompositeAlphaFlagBitsKHR compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
                //     VkCompositeAlphaFlagBitsKHR compositeAlphaFlags[4] = {
                //         VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                //         VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR,
                //         VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR,
                //         VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
                //     };
                //     for (uint32_t i = 0; i < sizeof(compositeAlphaFlags); i++) {
                //         if (surfCapabilities.supportedCompositeAlpha & compositeAlphaFlags[i]) {
                //             compositeAlpha = compositeAlphaFlags[i];
                //             break;
                //         }
                //     }


                //     VkSwapchainCreateInfoKHR swapchain_ci = {};
                //     swapchain_ci.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
                //     swapchain_ci.pNext = NULL;
                //     swapchain_ci.surface = surface;
                //     swapchain_ci.minImageCount = desiredNumberOfSwapChainImages;
                //     swapchain_ci.imageFormat = info.format;
                //     swapchain_ci.imageExtent.width = swapchainExtent.width;
                //     swapchain_ci.imageExtent.height = swapchainExtent.height;
                //     swapchain_ci.preTransform = preTransform;
                //     swapchain_ci.compositeAlpha = compositeAlpha;
                //     swapchain_ci.imageArrayLayers = 1;
                //     swapchain_ci.presentMode = swapchainPresentMode;
                //     swapchain_ci.oldSwapchain = VK_NULL_HANDLE;
                //     swapchain_ci.clipped = true;
                //     swapchain_ci.imageColorSpace = VK_COLORSPACE_SRGB_NONLINEAR_KHR;
                //     swapchain_ci.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
                //     swapchain_ci.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
                //     swapchain_ci.queueFamilyIndexCount = 0; !!! must be chosen one !!!
                //     swapchain_ci.pQueueFamilyIndices = NULL;

                //     Swapchain swap_chain;
                //     res = vkCreateSwapchainKHR(info.device, &swapchain_ci, NULL, &swap_chain);
                //     assert(res == VK_SUCCESS);

            //swapchain stuff END


            // image thingy START
    
                // res = vkGetSwapchainImagesKHR(info.device, info.swap_chain, &info.swapchainImageCount, NULL);
                // assert(res == VK_SUCCESS);

                // VkImage *swapchainImages = (VkImage *)malloc(info.swapchainImageCount * sizeof(VkImage));
                // assert(swapchainImages);
                // res = vkGetSwapchainImagesKHR(info.device, info.swap_chain, &info.swapchainImageCount, swapchainImages);
                // assert(res == VK_SUCCESS);

                // info.buffers.resize(info.swapchainImageCount);
                // for (uint32_t i = 0; i < info.swapchainImageCount; i++) {
                //     info.buffers[i].image = swapchainImages[i];
                // }
                // free(swapchainImages);


            // image thingy END


            //image view part START

                // for (uint32_t i = 0; i < info.swapchainImageCount; i++) {
                //     VkImageViewCreateInfo color_image_view = {};
                //     color_image_view.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
                //     color_image_view.pNext = NULL;
                //     color_image_view.flags = 0;
                //     color_image_view.image = info.buffers[i].image;
                //     color_image_view.viewType = VK_IMAGE_VIEW_TYPE_2D;
                //     color_image_view.format = info.format;
                //     color_image_view.components.r = VK_COMPONENT_SWIZZLE_R;
                //     color_image_view.components.g = VK_COMPONENT_SWIZZLE_G;
                //     color_image_view.components.b = VK_COMPONENT_SWIZZLE_B;
                //     color_image_view.components.a = VK_COMPONENT_SWIZZLE_A;
                //     color_image_view.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
                //     color_image_view.subresourceRange.baseMipLevel = 0;
                //     color_image_view.subresourceRange.levelCount = 1;
                //     color_image_view.subresourceRange.baseArrayLayer = 0;
                //     color_image_view.subresourceRange.layerCount = 1;

                //     res = vkCreateImageView(info.device, &color_image_view, NULL, &info.buffers[i].view);
                //     assert(res == VK_SUCCESS);
                // }

            //image view part END


            //vkDestroySurfaceKHR START

            let destroy_surface_function: vkDestroySurfaceKHR = 
                get_vk_function_with_null_vk_instance(vk_functions::DESTROY_SURFACE);

            destroy_surface_function(vk_instance, surface, ptr::null());

            println!("Surface was destroyed successfully!");            

            //vkDestroySurfaceKHR END


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