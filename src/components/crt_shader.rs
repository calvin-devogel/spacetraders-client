use iced::Rectangle;
use iced::mouse;
use iced::wgpu;
use iced::widget::shader::{self};
use std::mem;

#[derive(Debug, Clone, Copy)]
pub struct CrtOverlay {
    pub time: f32,
}

impl<Message> shader::Program<Message> for CrtOverlay {
    type State = ();
    type Primitive = CrtPrimitive;

    fn draw(&self, _state: &(), _cursor: mouse::Cursor, bounds: Rectangle) -> CrtPrimitive {
        CrtPrimitive {
            time: self.time,
            size: [bounds.width, bounds.height],
        }
    }
}

#[derive(Debug)]
pub struct CrtPrimitive {
    time: f32,
    size: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    time: f32,
    width: f32,
    height: f32,
    _pad: f32,
}

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl shader::Pipeline for Pipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("crt"),
            source: wgpu::ShaderSource::Wgsl(CRT_WGSL.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("crt uniforms"),
            size: mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("crt bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("crt bg"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("crt layout"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("crt pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group,
        }
    }
}

impl shader::Primitive for CrtPrimitive {
    type Pipeline = Pipeline;
    fn prepare(
        &self,
        pipeline: &mut Pipeline,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        _viewport: &shader::Viewport,
    ) {
        queue.write_buffer(
            &pipeline.uniform_buffer,
            0,
            bytemuck::bytes_of(&Uniforms {
                time: self.time,
                width: self.size[0],
                height: self.size[1],
                _pad: 0.0,
            }),
        );
    }

    fn render(
        &self,
        pipeline: &Pipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("crt pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_scissor_rect(
            clip_bounds.x,
            clip_bounds.y,
            clip_bounds.width,
            clip_bounds.height,
        );
        pass.set_pipeline(&pipeline.pipeline);
        pass.set_bind_group(0, &pipeline.bind_group, &[]);
        pass.draw(0..6, 0..1);
    }
}

const CRT_WGSL: &str = r#"
struct Uniforms {
    time:   f32,
    width:  f32,
    height: f32,
    _pad:   f32,
}
@group(0) @binding(0) var<uniform> u: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0), vec2( 1.0, -1.0), vec2(-1.0,  1.0),
        vec2( 1.0, -1.0), vec2( 1.0,  1.0), vec2(-1.0,  1.0),
    );
    return vec4(pos[idx], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_pos.xy / vec2(u.width, u.height);

    // Hard binary scanline: 1 dark row every 3 rows
    let scanline = select(0.0, 0.5, (u32(frag_pos.y) % 3u) == 0u);

    // Per-pixel grain, low amplitude
    let seed = frag_pos.xy + vec2(u.time * 7.3, u.time * 3.1);
    let grain = (fract(sin(dot(seed, vec2(12.9898, 78.233))) * 43758.5453) - 0.5) * 0.4;

    // Subtle flicker
    let flicker = sin(u.time * 9.0) * 0.005;

    // Vignette
    let v = uv * (1.0 - uv.yx);
    let vignette = 1.0 - clamp(pow(v.x * v.y * 14.0, 0.35), 0.0, 1.0);

    let darkness = clamp(scanline + vignette * 0.5 - grain - flicker, 0.0, 0.88);
    return vec4(0.0, 0.0, 0.0, darkness);
}
"#;
