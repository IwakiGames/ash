use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_timeline_semaphore::NAME;

#[derive(Clone)]
pub struct TimelineSemaphore {
    handle: vk::Device,
    fp: vk::khr_timeline_semaphore::DeviceFn,
}

impl TimelineSemaphore {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_timeline_semaphore::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html>
    #[inline]
    pub unsafe fn get_semaphore_counter_value(&self, semaphore: vk::Semaphore) -> VkResult<u64> {
        let mut value = 0;
        (self.fp.get_semaphore_counter_value_khr)(self.handle, semaphore, &mut value)
            .result_with_success(value)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html>
    #[inline]
    pub unsafe fn wait_semaphores(
        &self,
        wait_info: &vk::SemaphoreWaitInfo,
        timeout: u64,
    ) -> VkResult<()> {
        (self.fp.wait_semaphores_khr)(self.handle, wait_info, timeout).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html>
    #[inline]
    pub unsafe fn signal_semaphore(&self, signal_info: &vk::SemaphoreSignalInfo) -> VkResult<()> {
        (self.fp.signal_semaphore_khr)(self.handle, signal_info).result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_timeline_semaphore::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
