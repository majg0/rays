use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopProxy};

use crate::*;

pub struct CameraSystem {
    speed: f64,
}

impl CameraSystem {
    pub fn new() -> CameraSystem {
        CameraSystem { speed: 0.1 }
    }
}

impl Default for CameraSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl System for CameraSystem {
    fn on_event(
        &mut self,
        event: &Event<AppEvent>,
        _: &EventLoopProxy<AppEvent>,
        _: &mut ControlFlow,
        world: &World,
    ) {
        let mut camera = world.view.camera.borrow_mut();
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                camera.aspect_ratio = f64::from(size.width) / f64::from(size.height);
                camera.dirty = true;
            }

            Event::UserEvent(AppEvent::Action(Action::CamMove(v))) => {
                let v = self.speed * Vec3::new(v.x, 0.0, -v.y);
                camera.origin += v;
                camera.lookat += v;
                camera.dirty = true;
            }

            _ => {}
        }
    }

    fn on_frame_update(&mut self, world: &World) {
        world.view.camera.borrow_mut().refresh();
    }
}
