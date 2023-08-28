use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_win32_surface::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html>
#[derive(Clone)]
pub struct Win32Surface {
    handle: vk::Instance,
    fp: vk::khr_win32_surface::InstanceFn,
}

impl Win32Surface {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr_win32_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html>
    #[inline]
    pub unsafe fn create_win32_surface(
        &self,
        create_info: &vk::Win32SurfaceCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        (self.fp.create_win32_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        )
        .result_with_success(surface)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_win32_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        let b = (self.fp.get_physical_device_win32_presentation_support_khr)(
            physical_device,
            queue_family_index,
        );

        b > 0
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_win32_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
