//! This module contains crucial GPU-side structures, abstractions and
//! implementations for game's camera.

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::scene::camera::Camera;

/// Represents a GPU side of the game's [Camera].
pub struct GpuCamera {
    /// GPU buffer that camera will be writing into.
    pub buffer: Buffer,

    /// Bind group for the camera.
    pub bind_group: BindGroup,

    /// The layout of the camera's bind group.
    pub bind_group_layout: BindGroupLayout,
}

impl GpuCamera {
    /// Creates a new GPU-sided camera for the [Camera].
    pub fn new(device: &Device, camera: &Camera) -> Self {
        let uniform = CameraUniform::from_camera(camera);
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Renderer Camera Buffer"),
            contents: bytemuck::bytes_of(&uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,                       // Shader's input binding.
                visibility: ShaderStages::VERTEX, // We need camera only in vertex.
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform, // Our camera is represented as uniform.
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    /// Schedules the update for the camera, queueing current buffer write.
    pub fn schedule_update(&self, camera: &Camera, queue: &Queue) {
        let uniform = CameraUniform::from_camera(camera);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }
}

/// Represents the GPU side of the camera.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// The view and the projection matrix of the camera view.
    pub view_projection_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    /// Creates a new camera uniform from the given camera, extracting its
    /// view and projection matrices and multiplying it.
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            view_projection_matrix: camera
                .projection_matrix()
                .mul(&camera.view_matrix())
                .to_wgsl_matrix(),
        }
    }
}
