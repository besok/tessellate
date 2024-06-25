mod state;
mod vertex;
mod runner;
mod camera;


pub async fn run() {
    runner::run().await;
}
