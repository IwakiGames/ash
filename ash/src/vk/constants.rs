use crate::vk::definitions::*;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const UUID_SIZE: usize = 16;
pub const LUID_SIZE: usize = 8;
pub const MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const MAX_DESCRIPTION_SIZE: usize = 256;
pub const MAX_MEMORY_TYPES: usize = 32;
#[doc = "The maximum number of unique memory heaps, each of which supporting 1 or more memory types"]
pub const MAX_MEMORY_HEAPS: usize = 16;
pub const LOD_CLAMP_NONE: f32 = 1000.00;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const WHOLE_SIZE: u64 = !0;
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const TRUE: Bool32 = 1;
pub const FALSE: Bool32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const MAX_DRIVER_NAME_SIZE: usize = 256;
pub const MAX_DRIVER_INFO_SIZE: usize = 256;
pub const SHADER_UNUSED_KHR: u32 = !0;
pub const MAX_GLOBAL_PRIORITY_SIZE_KHR: usize = 16;
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: usize = 32;
pub const SHADER_UNUSED_NV: u32 = SHADER_UNUSED_KHR;
