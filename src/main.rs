struct App {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Option<std::sync::Arc<winit::window::Window>>,
    surface: Option<wgpu::Surface<'static>>,
    format: Option<wgpu::TextureFormat>,
    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl App {
    pub async fn new() -> Self {
        let instance = App::create_instance();
        let adapter = App::pick_adapter(&instance).await;
        let (device, queue) = App::request_device(&adapter).await;
        App {
            instance: instance,
            adapter: adapter,
            device: device,
            queue: queue,
            window: None,
            surface: None,
            format: None,
            render_pipeline: None,
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
        let adapters = instance.enumerate_adapters(wgpu::Backends::default()).await;
        for adapter in adapters {
            let score = App::score_adapter(&adapter);
            if score >= best_score {
                best_score = score;
                best_adapter = Some(adapter);
            }
        }
        if let Some(best_adapter_unwrapped) = best_adapter {
            return best_adapter_unwrapped;
        }
        panic!("No suitable adapter found!");
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

    fn pick_format(formats: &[wgpu::TextureFormat]) -> wgpu::TextureFormat {
        let supported = [
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        ];
        for format in supported {
            if formats.contains(&format) {
                return format;
            }
        }
        panic!(
            "No supported texture format was found!
The supported formats are {supported:?}.
The available formats are {formats:?}"
        );
    }

    fn create_render_pipeline(&self, wgsl_shader: &str) -> wgpu::RenderPipeline {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(wgsl_shader)),
            });

        let cache = App::create_pipeline_cache(&self.device);
        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                        format: self.format.unwrap(),
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
                self.render_impl(texture);
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                println!("Window is occluded, skipping frame...");
            }
            current_texture => {
                panic!("surface.get_current_texture() returned {current_texture:?}");
            }
        }
    }

    fn render_impl(&self, surface_texture: wgpu::SurfaceTexture) {
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut cmd_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap());
            render_pass.draw(
                std::range::Range { start: 0, end: 3 }.into(),
                std::range::Range { start: 0, end: 1 }.into(),
            );
        }
        self.queue.submit([cmd_encoder.finish()]);
        self.queue.present(surface_texture);
    }
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

        let capabilities = surface.get_capabilities(&self.adapter);

        let shader = std::fs::read("shader.wgsl").unwrap();
        let shader_str = str::from_utf8(shader.as_slice()).unwrap();

        self.format = Some(App::pick_format(&capabilities.formats));
        self.render_pipeline = Some(self.create_render_pipeline(shader_str));
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
