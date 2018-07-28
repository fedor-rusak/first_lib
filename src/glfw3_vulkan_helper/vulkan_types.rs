#![allow(non_camel_case_types, dead_code)]

macro_rules! vk_bitflags_wrapped {
    ($name: ident, $all: expr, $flag_type: ty) => {
        #[repr(C)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {flags: $flag_type}

        impl Default for $name{
            fn default() -> $name {
                $name {flags: 0}
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
                write!(f, "{}({:b})", stringify!($name), self.flags)
            }
        }

        impl $name {
            #[inline]
            pub fn empty() -> $name {
                $name {flags: 0}
            }

            #[inline]
            pub fn all() -> $name {
                $name {flags: $all}
            }

            #[inline]
            pub fn flags(self) -> $flag_type {
                self.flags
            }

            #[inline]
            pub fn from_flags(flags: $flag_type) -> Option<$name> {
                if flags & !$all == 0 {
                    Some($name {flags: flags})
                } else {
                    None
                }
            }

            #[inline]
            pub fn from_flags_truncate(flags: $flag_type) -> $name {
                $name {flags: flags & $all}
            }

            #[inline]
            pub fn is_empty(self) -> bool {
                self == $name::empty()
            }

            #[inline]
            pub fn is_all(self) -> bool {
                self & $name::all() == $name::all()
            }

            #[inline]
            pub fn intersects(self, other: $name) -> bool {
                self & other != $name::empty()
            }

            /// Returns true of `other` is a subset of `self`
            #[inline]
            pub fn subset(self, other: $name) -> bool {
                self & other == other
            }
        }

        impl BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name {flags: self.flags | rhs.flags }
            }
        }

        impl BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                *self = *self | rhs
            }
        }

        impl BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name {flags: self.flags & rhs.flags}
            }
        }

        impl BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                *self = *self & rhs
            }
        }

        impl BitXor for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name {flags: self.flags ^ rhs.flags}
            }
        }

        impl BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                *self = *self ^ rhs
            }
        }

        impl Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> $name {
                self & !rhs
            }
        }

        impl SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                *self = *self - rhs
            }
        }

        impl Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                self ^ $name::all()
            }
        }
    }
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

//this is for traits used by macros
use std::ops::*;
use std::fmt;

pub type c_void = ();
pub type c_char = i8;
pub type uint32_t = u32;
pub type size_t = usize;
pub type uint64_t = u64;
pub type uint8_t = u8;
pub type c_float = f32;
pub type int32_t = i32;

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
    pub flags: VkInstanceCreateFlags,
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

pub type Flags = uint32_t;

vk_bitflags_wrapped!(VkInstanceCreateFlags, 0b0, Flags);

#[allow(missing_copy_implementations)]
pub enum VkAllocationCallbacks {}


vk_define_handle!(VkInstance);

pub type vkCreateInstance = fn(*const VkInstanceCreateInfo, *const VkAllocationCallbacks, *mut VkInstance) -> *mut VkResult;