fn create_instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::default(),
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        backend_options: wgpu::BackendOptions::default(),
        display: None,
    })
}

fn score_adapter(adapter: &wgpu::Adapter) -> u32 {
    _ = adapter;
    0
}
async fn pick_adapter(instance: &wgpu::Instance) -> wgpu::Adapter {
    let mut best_score = 0u32;
    let mut best_adapter: Option<wgpu::Adapter> = None;
    for adapter in instance.enumerate_adapters(wgpu::Backends::default()).await {
        let score = score_adapter(&adapter);
        if score >= best_score {
            best_score = score;
            best_adapter = Some(adapter);
        }
    }
    best_adapter.unwrap()
}

async fn request_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .unwrap()
}

fn create_pipeline_cache(device: &wgpu::Device) -> Option<wgpu::PipelineCache> {
    if device
        .features()
        .features_wgpu
        .contains(wgpu::FeaturesWGPU::PIPELINE_CACHE)
    {
        unsafe {
            Some(
                device.create_pipeline_cache(&wgpu::PipelineCacheDescriptor {
                    label: None,
                    data: None,
                    fallback: false,
                }),
            )
        }
    } else {
        None
    }
}

fn create_render_pipeline(device: &wgpu::Device, wgsl_shader: &str) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(wgsl_shader)),
    });
    let cache = create_pipeline_cache(&device);
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: None,
                write_mask: wgpu::ColorWrites::all(),
            })],
        }),
        multiview_mask: None,
        cache: cache.as_ref(),
    })
}

#[tokio::main]
async fn main() {
    let instance = create_instance();
    let adapter = pick_adapter(&instance).await;
    let (device, queue) = request_device(&adapter).await;
    let shader = std::fs::read("shader.wgsl").unwrap();
    let shader_str = str::from_utf8(shader.as_slice()).unwrap();
    let render_pipeline = create_render_pipeline(&device, shader_str);
    _ = render_pipeline;
    _ = queue;
}
