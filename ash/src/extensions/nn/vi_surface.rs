use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::nn_vi_surface::NAME;

#[derive(Clone)]
pub struct ViSurface {
    handle: vk::Instance,
    fp: vk::nn_vi_surface::InstanceFn,
}

impl ViSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::nn_vi_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html>
    #[inline]
    pub unsafe fn create_vi_surface(
        &self,
        create_info: &vk::ViSurfaceCreateInfoNN,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        (self.fp.create_vi_surface_nn)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        )
        .result_with_success(surface)
    }

    #[inline]
    pub fn fp(&self) -> &vk::nn_vi_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
