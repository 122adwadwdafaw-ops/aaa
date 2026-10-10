//! Buffer creation that survives a failed allocation.
//!
//! wgpu's `DeviceExt::create_buffer_init` panics when the buffer it just created can't be
//! mapped, which is what happens when the driver fails the allocation (on an integrated GPU
//! with shared memory the failure can come as `ERROR_INITIALIZATION_FAILED` instead of an
//! out-of-memory error, #591). [`init`] has the same behaviour on success; on failure it returns
//! the invalid buffer after logging, so the failure stays what wgpu already made of it: an
//! error in the current error scope ([`crate::Gpu::within_memory`]) or the device's handler,
//! which retires the compositor and lets the frame fall back to the CPU.

use wgpu::util::BufferInitDescriptor;

/// A buffer holding `desc.contents`, padded to `COPY_BUFFER_ALIGNMENT`; never panics.
pub(crate) fn init(device: &wgpu::Device, desc: &BufferInitDescriptor<'_>) -> wgpu::Buffer {
    let unpadded = desc.contents.len();
    if unpadded == 0 {
        return device.create_buffer(&wgpu::BufferDescriptor { label: desc.label, size: 0, usage: desc.usage, mapped_at_creation: false });
    }
    let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;
    let padded = ((unpadded as wgpu::BufferAddress + align_mask) & !align_mask).max(wgpu::COPY_BUFFER_ALIGNMENT);
    let buffer = device.create_buffer(&wgpu::BufferDescriptor { label: desc.label, size: padded, usage: desc.usage, mapped_at_creation: true });
    match buffer.get_mapped_range_mut(..) {
        Ok(mut view) => view.slice(..unpadded).copy_from_slice(desc.contents),
        Err(e) => {
            log::error!("GPU buffer {:?} ({padded} bytes) could not be mapped at creation: {e}", desc.label);
            return buffer;
        }
    }
    buffer.unmap();
    buffer
}
