#[tokio::main]
async fn main() {
    if std::env::args().len() != 2 {
        println!("USAGE: game_of_life_wgpu <path-to-pbm>");
        return;
    }
    let mut app = game_of_life_wgpu::App::new(&std::env::args().last().unwrap()).await;
    app.run();
}
