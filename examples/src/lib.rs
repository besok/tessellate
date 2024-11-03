use env_logger::Builder;
use log::{info, LevelFilter};
use std::io::Error;

pub fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() {
}
