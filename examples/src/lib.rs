use env_logger::Builder;
use log::{info, LevelFilter};

pub fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() {
}