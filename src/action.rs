use crate::*;

#[derive(Debug)]
pub enum Action {
    XrayAdd,
    XraySub,
    XrayReset,
    CamMove(Vec2),
    CursorPos(Option<Vec2>),
    Quit,
}
