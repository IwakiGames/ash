use crate::prelude::*;
use crate::vk;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;
use std::ptr;

pub const NAME: &CStr = vk::khr_device_group_creation::NAME;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group_creation.html>
#[derive(Clone)]
pub struct DeviceGroupCreation {
    handle: vk::Instance,
    fp: vk::khr_device_group_creation::InstanceFn,
}

impl DeviceGroupCreation {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr_device_group_creation::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// Retrieve the number of elements to pass to [`enumerate_physical_device_groups()`][Self::enumerate_physical_device_groups()]
    #[inline]
    pub unsafe fn enumerate_physical_device_groups_len(&self) -> VkResult<usize> {
        let mut group_count = 0;
        (self.fp.enumerate_physical_device_groups_khr)(
            self.handle,
            &mut group_count,
            ptr::null_mut(),
        )
        .result_with_success(group_count as usize)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html>
    ///
    /// Call [`enumerate_physical_device_groups_len()`][Self::enumerate_physical_device_groups_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        out: &mut [vk::PhysicalDeviceGroupProperties],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.enumerate_physical_device_groups_khr)(self.handle, &mut count, out.as_mut_ptr())
            .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_device_group_creation::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
