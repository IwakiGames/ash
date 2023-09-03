use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_xlib_surface::NAME;

#[derive(Clone)]
pub struct XlibSurface {
    handle: vk::Instance,
    fp: vk::khr_xlib_surface::InstanceFn,
}

impl XlibSurface {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr_xlib_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_xlib_surface(
        &self,
        create_info: &vk::XlibSurfaceCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        (self.fp.create_xlib_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        )
        .result_with_success(surface)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_xlib_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        display: *mut vk::Display,
        visual_id: vk::VisualID,
    ) -> bool {
        let b = (self.fp.get_physical_device_xlib_presentation_support_khr)(
            physical_device,
            queue_family_index,
            display,
            visual_id,
        );

        b > 0
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_xlib_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
