#![allow(non_camel_case_types, dead_code)]

#[macro_export]
macro_rules! vk_make_version {
    ($major: expr, $minor: expr, $patch: expr) => ((($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32)
}

macro_rules! vk_define_handle{
    ($name: ident) => {
        #[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $name{
            ptr: *mut u8
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name{
            pub unsafe fn null() -> Self{
                $name{
                    ptr: ::std::ptr::null_mut()
                }
            }
        }
    }
}

pub type c_void = ();
pub type c_char = i8;
pub type uint32_t = u32;
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint8_t = u8;
pub type c_float = f32;
pub type int32_t = i32;

pub type Bool32 = uint32_t;
pub type DeviceSize = uint64_t;
pub type SampleMask = uint32_t;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VkStructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    LoaderInstanceCreateInfo = 47,
    LoaderDeviceCreateInfo = 48,
    XlibSurfaceCreateInfoKhr = 1000004000,
    XcbSurfaceCreateInfoKhr = 1000005000,
    MirSurfaceCreateInfoKhr = 1000007000,
    Win32SurfaceCreateInfoKhr = 1000009000,
    AndroidSurfaceCreateInfoKhr = 1000008000,
    WaylandSurfaceCreateInfoKhr = 1000006000,
    SwapchainCreateInfoKhr = 1000001000,
    PresentInfoKhr = 1000001001,
    DisplayPresentInfoKhr = 1000003000,
    DisplayModeCreateInfoKhr = 1000002000,
    DisplaySurfaceCreateInfoKhr = 1000002001,
    DebugReportCallbackCreateInfoExt = 1000011000,
}   

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorSurfaceLostKhr = -1000000000,
    ErrorNativeWindowInUseKhr = -1000000001,
    SuboptimalKhr = 1000001003,
    ErrorOutOfDateKhr = -1000001004,
    ErrorIncompatibleDisplayKhr = -1000003001,
    ErrorValidationFailedExt = -1000011001,
}


#[derive(Debug, Clone)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFlags,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: uint32_t,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: uint32_t,
    pub pp_enabled_extension_names: *const *const c_char,
}


#[derive(Debug, Clone)]
#[repr(C)]
pub struct VkApplicationInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: uint32_t,
    pub p_engine_name: *const c_char,
    pub engine_version: uint32_t,
    pub api_version: uint32_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VkFlags {
    Empty = 0
}

#[allow(missing_copy_implementations)]
pub enum VkAllocationCallbacks {}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DeviceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFlags,
    pub queue_create_info_count: uint32_t,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: uint32_t,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: uint32_t,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFlags,
    pub queue_family_index: uint32_t,
    pub queue_count: uint32_t,
    pub p_queue_priorities: *const c_float,
}

 #[repr(C)]
    pub struct PhysicalDeviceFeatures {
        pub robust_buffer_access: Bool32,
        pub full_draw_index_uint32: Bool32,
        pub image_cube_array: Bool32,
        pub independent_blend: Bool32,
        pub geometry_shader: Bool32,
        pub tessellation_shader: Bool32,
        pub sample_rate_shading: Bool32,
        pub dual_src_blend: Bool32,
        pub logic_op: Bool32,
        pub multi_draw_indirect: Bool32,
        pub draw_indirect_first_instance: Bool32,
        pub depth_clamp: Bool32,
        pub depth_bias_clamp: Bool32,
        pub fill_mode_non_solid: Bool32,
        pub depth_bounds: Bool32,
        pub wide_lines: Bool32,
        pub large_points: Bool32,
        pub alpha_to_one: Bool32,
        pub multi_viewport: Bool32,
        pub sampler_anisotropy: Bool32,
        pub texture_compression_etc2: Bool32,
        pub texture_compression_astc_ldr: Bool32,
        pub texture_compression_bc: Bool32,
        pub occlusion_query_precise: Bool32,
        pub pipeline_statistics_query: Bool32,
        pub vertex_pipeline_stores_and_atomics: Bool32,
        pub fragment_stores_and_atomics: Bool32,
        pub shader_tessellation_and_geometry_point_size: Bool32,
        pub shader_image_gather_extended: Bool32,
        pub shader_storage_image_extended_formats: Bool32,
        pub shader_storage_image_multisample: Bool32,
        pub shader_storage_image_read_without_format: Bool32,
        pub shader_storage_image_write_without_format: Bool32,
        pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
        pub shader_sampled_image_array_dynamic_indexing: Bool32,
        pub shader_storage_buffer_array_dynamic_indexing: Bool32,
        pub shader_storage_image_array_dynamic_indexing: Bool32,
        pub shader_clip_distance: Bool32,
        pub shader_cull_distance: Bool32,
        pub shader_float64: Bool32,
        pub shader_int64: Bool32,
        pub shader_int16: Bool32,
        pub shader_resource_residency: Bool32,
        pub shader_resource_min_lod: Bool32,
        pub sparse_binding: Bool32,
        pub sparse_residency_buffer: Bool32,
        pub sparse_residency_image2d: Bool32,
        pub sparse_residency_image3d: Bool32,
        pub sparse_residency2samples: Bool32,
        pub sparse_residency4samples: Bool32,
        pub sparse_residency8samples: Bool32,
        pub sparse_residency16samples: Bool32,
        pub sparse_residency_aliased: Bool32,
        pub variable_multisample_rate: Bool32,
        pub inherited_queries: Bool32,
}

vk_define_handle!(VkInstance);
vk_define_handle!(VkDevice);
vk_define_handle!(VkPhysicalDevice);

pub type vkCreateInstance = fn(*const VkInstanceCreateInfo, *const VkAllocationCallbacks, *mut VkInstance) -> VkResult;
pub type vkDestroyInstance =  fn(instance: VkInstance, *const VkAllocationCallbacks);

pub type vkCreateDevice = fn(physical_device: VkPhysicalDevice, *const DeviceCreateInfo, *const VkAllocationCallbacks, *mut VkDevice) -> VkResult;