use std::{borrow::Cow, mem::size_of_val};

use wgpu::{BufferUsages, DownlevelFlags, MapMode, PollType};
use wgpu_test::{apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters};

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    tests.push(GLSL_COMPONENT_ARGUMENT_INDEX);
}

#[apply(gpu_test!)]
static GLSL_COMPONENT_ARGUMENT_INDEX: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(
        TestParameters::default()
            .downlevel_flags(DownlevelFlags::COMPUTE_SHADERS)
            .limits(wgpu::Limits::downlevel_defaults()),
    )
    .run_async(|ctx| async move {
        let expected: [u32; 12] = [1, 0, 1, 5, 0, 1, 11, 0, 1, 3, 4, 2];
        let size = size_of_val(&expected) as u64;
        let module = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Glsl {
                    shader: Cow::Borrowed(include_str!(
                        "../../../../naga/tests/in/glsl/component-argument-index.comp"
                    )),
                    stage: naga::ShaderStage::Compute,
                    defines: &[],
                },
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                cache: None,
            });
        let output = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let readback = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let bindings = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: output.as_entire_binding(),
            }],
        });
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bindings, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
        ctx.queue.submit(Some(encoder.finish()));
        readback.slice(..).map_async(MapMode::Read, |_| ());
        ctx.async_poll(PollType::wait_indefinitely()).await.unwrap();
        let mapped = readback.slice(..).get_mapped_range().unwrap();
        let actual: &[u32] = bytemuck::cast_slice(&mapped);
        assert_eq!(actual, expected);
    });
