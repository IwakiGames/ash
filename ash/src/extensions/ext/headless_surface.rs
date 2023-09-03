use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext_headless_surface::NAME;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html>
#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: vk::ext_headless_surface::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ext_headless_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html>
    #[inline]
    pub unsafe fn create_headless_surface(
        &self,
        create_info: &vk::HeadlessSurfaceCreateInfoEXT,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        (self.fp.create_headless_surface_ext)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        )
        .result_with_success(surface)
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext_headless_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
