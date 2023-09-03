use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_android_surface::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_android_surface.html>
#[derive(Clone)]
pub struct AndroidSurface {
    handle: vk::Instance,
    fp: vk::khr_android_surface::InstanceFn,
}

impl AndroidSurface {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr_android_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_android_surface(
        &self,
        create_info: &vk::AndroidSurfaceCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        (self.fp.create_android_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        )
        .result_with_success(surface)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_android_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
