#[tokio::main]
async fn main() {
    let mut app = game_of_life_wgpu::App::new().await;
    app.run();
}
