use crate::*;

pub struct PickingSystem;

impl System for PickingSystem {
    fn on_frame_update(&mut self, _world: &World) {
        // *world.hovered_eid.borrow_mut() = world
        //     .mouse_pos
        //     .borrow()
        //     .map(|v| {
        //         let spheres = world.model.sphere.read();

        //         let hit = trace(
        //             &world.camera.borrow().get_ray(v.x, 1.0 - v.y),
        //             spheres,
        //             &world.view_settings.borrow(),
        //         );
        //     })
        //     .flatten();
    }
}
