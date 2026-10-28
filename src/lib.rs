const ROWS: usize = 32;
const COLS: usize = 64;

#[rustfmt::skip]
    const INITIAL_DATA_PRETTY: [[u8; COLS / 8 ]; ROWS] = [
        [0b01000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00100000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b11100000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
        [0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8, 0b00000000u8],
    ];

const INITIAL_DATA_RAW_SIZE: usize = COLS * ROWS / 8;

fn initial_data_raw() -> [u8; INITIAL_DATA_RAW_SIZE] {
    let mut raw: [u8; COLS * ROWS / 8] = INITIAL_DATA_PRETTY.as_flattened().try_into().unwrap();
    for byte_ref in raw.iter_mut() {
        let byte = *byte_ref;
        *byte_ref = 0;
        for i in 0..8 {
            *byte_ref |= ((byte >> i) & 1) << (7 - i);
        }
    }
    raw
}

pub struct App {
    context: Context,
    renderer: Renderer,
    window: Option<Window>,
}

impl App {
    pub async fn new() -> Self {
        let context = Context::new().await;
        let renderer = Renderer::new(&context).await;

        App {
            context,
            renderer,
            window: None,
        }
    }

    fn resize(&self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.update_size(&self.context, size);

        self.window.as_ref().unwrap().surface.configure(
            &self.context.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.renderer.format.unwrap(),
                color_space: wgpu::SurfaceColorSpace::Auto,
                width: size.width,
                height: size.height,
                desired_maximum_frame_latency: 2,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );
    }

    pub fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Window::new(&self.context, event_loop);

        self.renderer.create_pipeline(&self.context, &window);

        window.configure_surface(&self.context, self.renderer.format.unwrap());
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                if let Some(texture) = self.window.as_ref().unwrap().next_texture() {
                    self.renderer.render_frame(
                        &self.context,
                        &self.window.as_ref().unwrap(),
                        texture,
                    )
                }
            }
            winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                println!("Window resized to {width}x{height}.");
                self.resize(winit::dpi::PhysicalSize { width, height })
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.as_ref().unwrap().window.request_redraw();
    }
}

struct Context {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Context {
    async fn new() -> Self {
        let instance = Context::create_instance();
        let adapter = Context::pick_adapter(&instance).await;
        let (device, queue) = Context::request_device(&adapter).await;

        Context {
            instance,
            adapter,
            device,
            queue,
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
            let score = Context::score_adapter(&adapter);
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
}

struct Renderer {
    ssbos: [wgpu::Buffer; 2],
    compute_bind_groups: [wgpu::BindGroup; 2],
    compute_pipeline: wgpu::ComputePipeline,
    render_bind_group_layout: wgpu::BindGroupLayout,
    render_bind_groups: [wgpu::BindGroup; 2],
    format: Option<wgpu::TextureFormat>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    frame_index: usize,
}

impl Renderer {
    async fn new(context: &Context) -> Self {
        let shader = Renderer::create_shader(&context.device, "shaders/compute.wgsl");
        let ssbos = [
            Renderer::create_ssbo(&context.device, &context.queue),
            Renderer::create_ssbo(&context.device, &context.queue),
        ];

        let compute_bind_group_layout = Renderer::create_compute_bind_group_layout(&context.device);
        let compute_bind_groups = [
            Renderer::create_compute_bind_group(
                &context.device,
                &compute_bind_group_layout,
                &ssbos[0],
                &ssbos[1],
            ),
            Renderer::create_compute_bind_group(
                &context.device,
                &compute_bind_group_layout,
                &ssbos[1],
                &ssbos[0],
            ),
        ];
        let compute_pipeline =
            Renderer::create_compute_pipeline(&context.device, &shader, &compute_bind_group_layout);

        let render_bind_group_layout = Renderer::create_render_bind_group_layout(&context.device);
        let render_bind_groups = [
            Renderer::create_render_bind_group(
                &context.device,
                &render_bind_group_layout,
                &ssbos[0],
            ),
            Renderer::create_render_bind_group(
                &context.device,
                &render_bind_group_layout,
                &ssbos[1],
            ),
        ];

        Renderer {
            ssbos,
            compute_pipeline,
            compute_bind_groups,
            render_bind_group_layout,
            render_bind_groups,
            format: None,
            render_pipeline: None,
            frame_index: 0,
        }
    }

    fn create_shader(device: &wgpu::Device, path: &str) -> wgpu::ShaderModule {
        let cwd = std::env::current_dir().unwrap();
        println!("cwd: {cwd:?}");
        println!("Compiling shader {path:?}...");
        let shader = std::fs::read(path).unwrap();
        let shader_str = str::from_utf8(shader.as_slice()).unwrap();

        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(shader_str)),
        })
    }

    fn create_compute_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }

    fn create_compute_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        in_ssbo: &wgpu::Buffer,
        out_ssbo: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: in_ssbo,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: out_ssbo,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        })
    }

    fn create_compute_pipeline_layout(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(bind_group_layout)],
            immediate_size: 0,
        })
    }

    fn create_compute_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::ComputePipeline {
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&Renderer::create_compute_pipeline_layout(
                device,
                bind_group_layout,
            )),
            module: shader,
            entry_point: None,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &[("0", ROWS as f64), ("1", COLS as f64)],
                zero_initialize_workgroup_memory: false,
            },
            cache: None,
        })
    }

    fn create_render_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    fn create_render_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        ssbo: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: ssbo,
                    offset: 0,
                    size: None,
                }),
            }],
        })
    }

    fn create_ssbo(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Buffer {
        let ssbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (INITIAL_DATA_RAW_SIZE + size_of::<u32>() * 2) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&ssbo, 0, &initial_data_raw());
        ssbo
    }

    fn create_pipeline_cache(context: &Context) -> Option<wgpu::PipelineCache> {
        if context
            .device
            .features()
            .features_wgpu
            .contains(wgpu::FeaturesWGPU::PIPELINE_CACHE)
        {
            unsafe {
                Some(
                    context
                        .device
                        .create_pipeline_cache(&wgpu::PipelineCacheDescriptor {
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

    fn create_render_pipeline_layout(&self, context: &Context) -> wgpu::PipelineLayout {
        context
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&self.render_bind_group_layout)],
                immediate_size: 0,
            })
    }

    fn create_render_pipeline(
        &self,
        context: &Context,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        let cache = Renderer::create_pipeline_cache(context);
        context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&self.create_render_pipeline_layout(context)),
                vertex: wgpu::VertexState {
                    module: shader,
                    entry_point: None,
                    compilation_options: wgpu::PipelineCompilationOptions {
                        constants: &[("0", ROWS as f64), ("1", COLS as f64)],
                        zero_initialize_workgroup_memory: false,
                    },
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: None,
                    compilation_options: wgpu::PipelineCompilationOptions {
                        constants: &[("0", ROWS as f64), ("1", COLS as f64)],
                        zero_initialize_workgroup_memory: false,
                    },
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

    pub fn update_size(&self, context: &Context, size: winit::dpi::PhysicalSize<u32>) {
        let (width, height) = if size.height > size.width * ROWS as u32 / COLS as u32 {
            (size.width, size.width * ROWS as u32 / COLS as u32)
        } else {
            (size.height * COLS as u32 / ROWS as u32, size.height)
        };

        let size_bytes = [
            (width >> 0) as u8,
            (width >> 8) as u8,
            (width >> 16) as u8,
            (width >> 24) as u8,
            (height >> 0) as u8,
            (height >> 8) as u8,
            (height >> 16) as u8,
            (height >> 24) as u8,
        ];
        context
            .queue
            .write_buffer(&self.ssbos[0], INITIAL_DATA_RAW_SIZE as u64, &size_bytes);
        context
            .queue
            .write_buffer(&self.ssbos[1], INITIAL_DATA_RAW_SIZE as u64, &size_bytes);
    }

    fn render_frame(
        &mut self,
        context: &Context,
        window: &Window,
        surface_texture: wgpu::SurfaceTexture,
    ) {
        println!("Rendering frame #{}...", self.frame_index);

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut cmd_encoder = context
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

            render_pass.set_bind_group(0, &self.render_bind_groups[self.frame_index % 2], &[]);
            render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap());

            render_pass.draw(
                std::range::Range { start: 0, end: 3 }.into(),
                std::range::Range { start: 0, end: 1 }.into(),
            );
        }

        {
            let mut compute_pass = cmd_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_groups[self.frame_index % 2], &[]);
            compute_pass.dispatch_workgroups((COLS * ROWS / 32) as u32, 1, 1);
        }

        context.queue.submit([cmd_encoder.finish()]);
        window.window.pre_present_notify();
        context.queue.present(surface_texture);

        self.frame_index += 1;
    }

    fn create_pipeline(&mut self, context: &Context, target: &Window) {
        let capabilities = target.surface.get_capabilities(&context.adapter);

        let shader = Renderer::create_shader(&context.device, "shaders/graphics.wgsl");

        self.format = Some(Renderer::pick_format(&capabilities.formats));
        self.render_pipeline = Some(self.create_render_pipeline(context, &shader));
    }
}

struct Window {
    window: std::sync::Arc<winit::window::Window>,
    surface: wgpu::Surface<'static>,
}

impl Window {
    fn new(context: &Context, event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let window = std::sync::Arc::new(
            event_loop
                .create_window(winit::window::WindowAttributes::default())
                .unwrap(),
        );

        let surface = context.instance.create_surface(window.clone()).unwrap();

        window.request_redraw();

        Window { window, surface }
    }

    fn configure_surface(&self, context: &Context, format: wgpu::TextureFormat) {
        self.surface.configure(
            &context.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: format,
                color_space: wgpu::SurfaceColorSpace::Srgb,
                width: self.window.inner_size().width,
                height: self.window.inner_size().height,
                present_mode: wgpu::PresentMode::AutoVsync,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        )
    }

    fn next_texture(&self) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => {
                return Some(texture);
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return None;
            }
            current_texture => {
                panic!("surface.get_current_texture() returned {current_texture:?}");
            }
        }
    }
}
