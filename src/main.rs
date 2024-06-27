use winit::error::EventLoopError;
use tessellate::gpu::run;

fn main() {
    pollster::block_on(run());
}
