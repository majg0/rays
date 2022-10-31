mod event;
mod renderer;
mod settings;

use crate::*;

pub use event::*;
pub use renderer::*;
pub use settings::*;

use atomic_refcell::AtomicRefCell;

pub struct View {
    pub camera: AtomicRefCell<Camera>,
    pub settings: AtomicRefCell<ViewSettings>,
    pub renderer: AtomicRefCell<Renderer>,
    pub window_physical_size: AtomicRefCell<Vec2>,
    pub mouse_pos: AtomicRefCell<Option<Vec2>>,
    pub hovered_eid: AtomicRefCell<Option<u32>>,
}

impl View {
    pub fn new(
        camera: Camera,
        settings: ViewSettings,
        renderer: Renderer,
        window_physical_size: Vec2,
    ) -> View {
        View {
            camera: AtomicRefCell::new(camera),
            settings: AtomicRefCell::new(settings),
            renderer: AtomicRefCell::new(renderer),
            window_physical_size: AtomicRefCell::new(window_physical_size),
            mouse_pos: AtomicRefCell::new(None),
            hovered_eid: AtomicRefCell::new(None),
        }
    }
}
