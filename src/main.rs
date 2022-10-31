use std::io::Write;
use std::time::Instant;

use ron::ser::to_writer;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub use action::*;
pub use ecs::*;
pub use hittable::*;
pub use math::*;
pub use model::*;
pub use system::*;
pub use view::*;
pub use world::*;

mod action;
mod ecs;
mod hittable;
mod math;
mod model;
mod system;
mod view;
mod world;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 180;

fn main() {
    // Window
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let elp = event_loop.create_proxy();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(f64::from(WIDTH) * 2.0, f64::from(HEIGHT) * 2.0);
        WindowBuilder::new()
            .with_title("Rays")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // World

    // # Input
    //
    // 1. ECS
    // 2. Camera + Position + Velocity
    // 3. On input -> Create/Delete Velocity component from camera entity
    // 4. Input mappings for camera controls

    let mut world = {
        let size = window.inner_size();

        World::new(View::new(
            Camera::new(
                Point3::new(2.0, 32.0, 32.0),
                Point3::new(2.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                70.0,
                f64::from(WIDTH) / f64::from(HEIGHT),
            ),
            ViewSettings::new(),
            Renderer::new(&window, WIDTH, HEIGHT).expect("Unable to init Renderer"),
            Vec2::new(f64::from(size.width), f64::from(size.height)),
        ))
    };

    let mut systems: Vec<Box<dyn System>> = vec![
        Box::new(QuitSystem),
        Box::new(FpsSystem::new()),
        Box::new(InputSystem::new()),
        Box::new(XraySystem),
        Box::new(CameraSystem::new()), // dep: input
        Box::new(PickingSystem),       // dep: cam, input
        Box::new(RenderSystem),        // dep: cam, picking, hit settings
    ];

    // timing
    const UPDATE_FPS: f64 = 5.0;
    const TIME_STEP: f64 = 1.0 / UPDATE_FPS;
    const SLOWDOWN_FACTOR: f64 = 4.0;
    let mut ticks_simulated: u32 = 0;
    let mut last_updated = Instant::now();
    let mut time_available = 0.0;
    let time_speed = 1.0;

    // logs
    let mut f_model = std::fs::File::create("log/m.ronlog").unwrap();
    let mut f_view = std::fs::File::create("log/v.ronlog").unwrap();
    to_writer(&f_model, &world.model).unwrap();
    writeln!(f_model).unwrap();

    // model
    {
        let Model {
            entity,
            position,
            velocity,
            sphere,
        } = &mut world.model;

        let _cam = entity.alloc();
        let somedude = entity.alloc();

        {
            let mut p = position.insert();
            p.insert(somedude, Vec3::ZERO);
        }
        {
            let mut v = velocity.insert();
            v.insert(somedude, Vec3::ONE);
        }
        {
            let mut s = sphere.insert();
            s.insert(
                entity.alloc(),
                Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5, 0),
            );
            s.insert(
                entity.alloc(),
                Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.5, 1),
            );
            s.insert(
                entity.alloc(),
                Sphere::new(Point3::new(1.0, 0.0, 0.0), 0.5, 2),
            );
            s.insert(
                entity.alloc(),
                Sphere::new(Point3::new(0.0, -100.5, 0.0), 100.0, 3),
            );
        }
    }

    // systems
    let mut simulators: Vec<fn(&Model)> = Vec::new();

    let movement_sim = |m: &Model| {
        let mut pos = m.position.write();
        let vel = m.velocity.read();
        for e in iterate(&mut [&m.entity, &m.position, &m.velocity]) {
            pos[e] += TIME_STEP * vel[e];
        }
    };

    simulators.push(movement_sim);

    // 1. system events are processed and turned into actions.
    //    Actions come in two flavors: model actions and view actions.
    //    Model actions are applied during simulation.
    //    View actions are applied during rendering.
    //    Both action types are collected into the current frame of its corresponding type, for
    //    later processing.
    //
    // 2. upon MainEventsCleared, we check if it's time for a simulation step and that we have
    //    received the remote actions from the server for this frame.
    //
    //    If ok, the model action frame (MAF) is considered complete.
    //    We send the MAF off to the server and keep a local copy for latency hiding until we receive
    //    the authoritative remote actions for this particular frame.
    //    Next, we perform a simulation step:
    //
    //    prev = current.clone();
    //    optimistic = current.clone();
    //    optimistic = simulate(optimistic, remote_actions + local_actions);
    //    current = simulate(current, remote_actions);
    //
    //    This way, the current state is continuously updated to contain the true server state.
    //    This marks the end of the simulation step.
    //
    //    Regardless of performing a simulation step or not, we render:
    //    1. The previous and the optimistic states are blended between with an interpolation factor proportional
    //    to how long ago the previous frame occurred, yielding equally smooth motion regardless of FPS.

    event_loop.run(move |event, _, control_flow| {
        systems
            .iter_mut()
            .for_each(|s| s.on_event(&event, &elp, control_flow, &world));

        match &event {
            Event::NewEvents(_) => {
                systems.iter_mut().for_each(|s| s.on_frame_start(&world));
            }

            Event::MainEventsCleared => {
                systems.iter_mut().for_each(|s| s.on_frame_update(&world));

                {
                    let now = Instant::now();
                    // NOTE: when running on a really slow computer and the simulation step takes too long,
                    // slow down game time to 1.0 / SLOWDOWN_FACTOR, in order to decrease simulation work.
                    // TODO: needs to be communicated to the server (and oh btw we don't even have one yet :D)
                    time_available += ((now - last_updated).as_secs_f64() * time_speed)
                        .min(1.0 / SLOWDOWN_FACTOR);
                    last_updated = now;
                }

                while time_available >= TIME_STEP {
                    let _time_simulated = ticks_simulated as f64 * TIME_STEP;

                    world.prev_model = world.model.clone();

                    // TODO: "simulate"
                    {
                        // TODO: callback with atomic refs

                        // TODO: cleanup in storages after deallocs

                        simulators.iter().for_each(|f| {
                            f(&world.model);
                        });
                    }

                    time_available -= TIME_STEP;
                    ticks_simulated += 1;
                    to_writer(&f_model, &world.model).unwrap();
                    writeln!(f_model).unwrap();
                }

                let t = time_available / TIME_STEP;
                let m = world.prev_model.lerp(&world.model, t);

                // TODO: "render"
                to_writer(&f_view, &m).unwrap();
                writeln!(f_view).unwrap();
            }

            Event::RedrawEventsCleared => {
                systems.iter_mut().for_each(|s| s.on_frame_end(&world));
            }

            _ => {}
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) {
                Action::Quit.send(&elp);
            }

            let mut cam_move = Vec2::zero();
            if input.key_held(VirtualKeyCode::Left) || input.key_held(VirtualKeyCode::H) {
                cam_move.x -= 1.0;
            }
            if input.key_held(VirtualKeyCode::Up) || input.key_held(VirtualKeyCode::K) {
                cam_move.y += 1.0;
            }
            if input.key_held(VirtualKeyCode::Right) || input.key_held(VirtualKeyCode::L) {
                cam_move.x += 1.0;
            }
            if input.key_held(VirtualKeyCode::Down) || input.key_held(VirtualKeyCode::J) {
                cam_move.y -= 1.0;
            }
            if cam_move != Vec2::zero() {
                Action::CamMove(cam_move).send(&elp);
            }
        }
    });
}
