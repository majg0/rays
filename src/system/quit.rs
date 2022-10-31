use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopProxy};

use crate::*;

pub struct QuitSystem;

impl System for QuitSystem {
    fn on_event(
        &mut self,
        event: &Event<AppEvent>,
        _: &EventLoopProxy<AppEvent>,
        cf: &mut ControlFlow,
        _: &World,
    ) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    *cf = ControlFlow::Exit;
                }
                _ => {}
            },

            Event::UserEvent(AppEvent::Action(Action::Quit)) => {
                *cf = ControlFlow::Exit;
            }

            _ => {}
        }
    }
}
