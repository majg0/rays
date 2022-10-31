use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoopProxy};

use crate::*;

mod camera;
mod fps;
mod input;
mod picking;
mod quit;
mod render;
mod xray;

pub use camera::*;
pub use fps::*;
pub use input::*;
pub use picking::*;
pub use quit::*;
pub use render::*;
pub use xray::*;

pub trait System {
    fn on_event(
        &mut self,
        _: &Event<AppEvent>,
        _: &EventLoopProxy<AppEvent>,
        _: &mut ControlFlow,
        _: &World,
    ) {
    }

    fn on_frame_start(&mut self, _: &World) {}

    fn on_frame_update(&mut self, _: &World) {}

    fn on_frame_end(&mut self, _: &World) {}
}
