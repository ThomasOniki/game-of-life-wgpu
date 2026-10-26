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

#[tokio::main]
async fn main() {
    let instance = create_instance();
    let adapter = pick_adapter(&instance).await;
    let (device, queue) = request_device(&adapter).await;
    _ = device;
    _ = queue;
}
