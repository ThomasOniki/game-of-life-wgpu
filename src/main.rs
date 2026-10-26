struct App {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    window: Option<std::sync::Arc<winit::window::Window>>,
    surface: Option<wgpu::Surface<'static>>,
}

impl App {
    pub async fn new() -> Self {
        let instance = App::create_instance();
        let adapter = App::pick_adapter(&instance).await;
        let (device, queue) = App::request_device(&adapter).await;
        let shader = std::fs::read("shader.wgsl").unwrap();
        let shader_str = str::from_utf8(shader.as_slice()).unwrap();
        let render_pipeline = App::create_render_pipeline(&device, shader_str);
        App {
            instance: instance,
            adapter: adapter,
            device: device,
            queue: queue,
            render_pipeline: render_pipeline,
            window: None,
            surface: None,
        }
    }

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
            let score = App::score_adapter(&adapter);
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
        let cache = App::create_pipeline_cache(&device);
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

    fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }

    fn render(&self) {
        match self.surface.as_ref().unwrap().get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => {
                println!("Drawing frame...");
                self.render_impl(
                    texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                );
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                println!("Window is occluded, skipping frame...");
            }
            current_texture => {
                panic!("surface.get_current_texture() returned {current_texture:?}");
            }
        }
    }

    fn render_impl(&self, texture_view: wgpu::TextureView) {}
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = std::sync::Arc::new(
            event_loop
                .create_window(winit::window::WindowAttributes::default())
                .unwrap(),
        );
        let surface = self.instance.create_surface(window.clone()).unwrap();
        surface.configure(
            &self.device,
            &surface
                .get_default_config(
                    &self.adapter,
                    window.inner_size().width,
                    window.inner_size().height,
                )
                .unwrap(),
        );
        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == winit::event::WindowEvent::RedrawRequested {
            self.render();
        }
    }
}

#[tokio::main]
async fn main() {
    let mut app = App::new().await;
    app.run();
}
