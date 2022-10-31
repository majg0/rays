use winit::event_loop::EventLoopProxy;

use crate::*;

#[derive(Debug)]
pub enum AppEvent {
    // Input(Input),
    Action(Action),
}

pub trait UserEvent: Sized {
    fn send(self, p: &EventLoopProxy<AppEvent>);
}

impl UserEvent for Action {
    fn send(self, p: &EventLoopProxy<AppEvent>) {
        p.send_event(AppEvent::Action(self)).unwrap();
    }
}

// impl UserEvent for Input {
//     fn send(self, p: &EventLoopProxy<AppEvent>) {
//         p.send_event(AppEvent::Input(self)).unwrap();
//     }
// }
