use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;
use std::ptr;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html>
#[derive(Clone)]
pub struct ExternalSemaphoreWin32 {
    handle: vk::Device,
    fp: vk::khr_external_semaphore_win32::DeviceFn,
}

impl ExternalSemaphoreWin32 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_external_semaphore_win32::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_semaphore_win32_handle(
        &self,
        import_info: &vk::ImportSemaphoreWin32HandleInfoKHR,
    ) -> VkResult<()> {
        (self.fp.import_semaphore_win32_handle_khr)(self.handle, import_info).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_semaphore_win32_handle(
        &self,
        get_info: &vk::SemaphoreGetWin32HandleInfoKHR,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = ptr::null_mut();
        (self.fp.get_semaphore_win32_handle_khr)(self.handle, get_info, &mut handle)
            .result_with_success(handle)
    }

    pub const NAME: &'static CStr = vk::khr_external_semaphore_win32::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::khr_external_semaphore_win32::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
