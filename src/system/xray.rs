use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoopProxy};

use crate::*;

pub struct XraySystem;

impl System for XraySystem {
    fn on_event(
        &mut self,
        event: &Event<AppEvent>,
        _: &EventLoopProxy<AppEvent>,
        _: &mut ControlFlow,
        world: &World,
    ) {
        if let Event::UserEvent(AppEvent::Action(action)) = event {
            match action {
                Action::XraySub => {
                    let mut s = world.view.settings.borrow_mut();
                    if s.xray > 0 {
                        s.xray -= 1;
                    }
                }

                Action::XrayAdd => {
                    world.view.settings.borrow_mut().xray += 1;
                }

                Action::XrayReset => {
                    world.view.settings.borrow_mut().xray = 0;
                }

                _ => {}
            }
        }
    }
}
