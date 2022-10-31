use winit::event::{Event, KeyboardInput, ModifiersState, Touch, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopProxy};

use crate::*;

// #[derive(Debug)]
// pub enum Input {
//     Key(VirtualKeyCode, ElementState),
//     Char(char),
//     MouseButton(MouseButton, ElementState),
//     MousePos { p: Vec2, dp: Vec2 },
// }

// move cam (4 x continuous)
// xray (3 x discrete)
// quit (1 x discrete)
// aim (2 x continuous)
// pick (1 x discrete)

// events -> [t] -> event

pub struct InputState {
    pub modifiers: ModifiersState,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            modifiers: ModifiersState::empty(),
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputSystem {
    state: InputState,
}

impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem {
            state: InputState::new(),
        }
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl System for InputSystem {
    fn on_event(
        &mut self,
        event: &Event<AppEvent>,
        elp: &EventLoopProxy<AppEvent>,
        _: &mut ControlFlow,
        world: &World,
    ) {
        // let map = HashMap::new();
        // map.insert(VirtualKeyCode::Escape, Action::Quit);
        // map.insert(VirtualKeyCode::Q, Action::Quit);
        // map.insert(VirtualKeyCode::Left, Action::CamLeft);
        // map.insert(VirtualKeyCode::H, Action::CamLeft);
        // map.insert(VirtualKeyCode::Up, Action::CamUp);
        // map.insert(VirtualKeyCode::K, Action::CamUp);
        // map.insert(VirtualKeyCode::Right, Action::CamRight);
        // map.insert(VirtualKeyCode::L, Action::CamRight);
        // map.insert(VirtualKeyCode::Down, Action::CamDown);
        // map.insert(VirtualKeyCode::J, Action::CamDown);

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::ReceivedCharacter(char) => {
                    println!("ReceivedCharacter {:?}", char);
                    // TODO: move to keybindings
                    match char {
                        '-' => {
                            Action::XraySub.send(elp);
                        }
                        '+' => {
                            Action::XrayAdd.send(elp);
                        }
                        ' ' => {
                            Action::XrayReset.send(elp);
                        }
                        _ => {}
                    }
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    println!("KeyboardInput {:?} {:?}", state, key);
                }

                WindowEvent::ModifiersChanged(state) => {
                    self.state.modifiers = *state;
                    println!("ModifiersChanged {:?}", state);
                }

                WindowEvent::CursorMoved { position, .. } => {
                    let p = Vec2::new(position.x, position.y)
                        / *world.view.window_physical_size.borrow();
                    let mut mp = world.view.mouse_pos.borrow_mut();
                    let dp = mp.map(|p0| p - p0);
                    println!("CursorMoved {:?} {:?}", p, dp);

                    // TODO: move to keybinding
                    Action::CursorPos(Some(p)).send(elp);

                    *mp = Some(p);
                }

                WindowEvent::CursorLeft { .. } => {
                    *world.view.mouse_pos.borrow_mut() = None;
                }

                WindowEvent::MouseWheel { delta, phase, .. } => {
                    println!("MouseWheel {:?} {:?}", delta, phase);
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    println!("MouseInput {:?} {:?}", button, state);
                }

                WindowEvent::AxisMotion { axis, value, .. } => {
                    println!("AxisMotion {:?} {:?}", axis, value);
                }

                WindowEvent::Touch(Touch {
                    phase,
                    location,
                    id,
                    ..
                }) => {
                    let p = Vec2::new(location.x, location.y)
                        / *world.view.window_physical_size.borrow();
                    println!("Touch {:?} {:?} {:?}", phase, p, id);
                }

                _ => {}
            }
        }
    }
}
