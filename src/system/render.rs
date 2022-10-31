use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopProxy};

use crate::*;

pub struct RenderSystem;

impl System for RenderSystem {
    fn on_event(
        &mut self,
        event: &Event<AppEvent>,
        _: &EventLoopProxy<AppEvent>,
        _: &mut ControlFlow,
        world: &World,
    ) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Resized(size) => {
                    *world.view.window_physical_size.borrow_mut() =
                        Vec2::new(f64::from(size.width), f64::from(size.height));
                    world
                        .view
                        .renderer
                        .borrow_mut()
                        .resize(size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged {
                    new_inner_size: size,
                    ..
                } => {
                    *world.view.window_physical_size.borrow_mut() =
                        Vec2::new(f64::from(size.width), f64::from(size.height));
                    world
                        .view
                        .renderer
                        .borrow_mut()
                        .resize(size.width, size.height);
                }
                _ => {}
            }
        }
    }

    fn on_frame_update(&mut self, world: &World) {
        world.view.renderer.borrow_mut().render(world);
    }
}
