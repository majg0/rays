use pixels::{Error, Pixels, SurfaceTexture};
use rayon::prelude::*;
use winit::window::Window;

use crate::*;

pub struct Renderer {
    pub pixels: Pixels,
    pub buf_width: u32,
    pub buf_height: u32,
}

enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
const GRID_SIDE: i32 = 32;
const GRID_SIDE_F: f64 = GRID_SIDE as f64;
const GRID_SIDE_2: i32 = GRID_SIDE * GRID_SIDE;
const GRID_SIDE_3: i32 = GRID_SIDE * GRID_SIDE_2;

type Grid = [bool; GRID_SIDE_3 as usize];

fn ray_box(ray: &Ray) -> Option<(f64, f64, Axis)> {
    let t_ymin;
    let t_ymax;
    let t_zmin;
    let t_zmax;
    let mut t_min;
    let mut t_max;
    let mut axis = Axis::X;

    let x_inv_dir = 1.0 / ray.direction.x;
    if x_inv_dir >= 0.0 {
        t_min = (-ray.origin.x) * x_inv_dir;
        t_max = (GRID_SIDE_F - ray.origin.x) * x_inv_dir;
    } else {
        t_min = (GRID_SIDE_F - ray.origin.x) * x_inv_dir;
        t_max = (-ray.origin.x) * x_inv_dir;
    }

    let y_inv_dir = 1.0 / ray.direction.y;
    if y_inv_dir >= 0.0 {
        t_ymin = (-ray.origin.y) * y_inv_dir;
        t_ymax = (GRID_SIDE_F - ray.origin.y) * y_inv_dir;
    } else {
        t_ymin = (GRID_SIDE_F - ray.origin.y) * y_inv_dir;
        t_ymax = (-ray.origin.y) * y_inv_dir;
    }

    if t_min > t_ymax || t_ymin > t_max {
        return None;
    }
    if t_ymin > t_min {
        t_min = t_ymin;
        axis = Axis::Y;
    }
    if t_ymax < t_max {
        t_max = t_ymax;
    }

    let z_inv_dir = 1.0 / ray.direction.z;
    if z_inv_dir >= 0.0 {
        t_zmin = (-ray.origin.z) * z_inv_dir;
        t_zmax = (GRID_SIDE_F - ray.origin.z) * z_inv_dir;
    } else {
        t_zmin = (GRID_SIDE_F - ray.origin.z) * z_inv_dir;
        t_zmax = (-ray.origin.z) * z_inv_dir;
    }

    if t_min > t_zmax || t_zmin > t_max {
        return None;
    }
    if t_zmin > t_min {
        t_min = t_zmin;
        axis = Axis::Z;
    }
    if t_zmax < t_max {
        t_max = t_zmax;
    }
    Some((t_min, t_max, axis))
}

struct TerrainHit {
    index: IVec3,
    normal: Vec3,
    t: f64,
    u: f64,
    v: f64,
}

impl TerrainHit {
    pub fn new(index: IVec3, normal: Vec3, t: f64, u: f64, v: f64) -> TerrainHit {
        TerrainHit {
            index,
            normal,
            t,
            u,
            v,
        }
    }
}

fn amanatides_woo(
    ray: &Ray,
    t0: f64,
    t1: f64,
    grid: &Grid,
    view_settings: &ViewSettings,
) -> Option<TerrainHit> {
    let (mut t_min, mut t_max, mut axis) = ray_box(ray)?;

    t_min = t_min.max(t0);
    t_max = t_max.min(t1) - 0.001; // NOTE: the subtraction ensures ending within bounds
    let mut t = t_min;

    let start = ray.origin + t_min * ray.direction;
    // NOTE: the clamping ensures numerical stability around edges
    let mut i = IVec3::new(
        (start.x as i32).clamp(0, GRID_SIDE - 1),
        (start.y as i32).clamp(0, GRID_SIDE - 1),
        (start.z as i32).clamp(0, GRID_SIDE - 1),
    );

    let step_x;
    let t_dx;
    let mut t_max_x;
    if ray.direction.x > 0.0 {
        step_x = 1;
        t_dx = 1.0 / ray.direction.x;
        t_max_x = t_min + ((i.x + 1) as f64 - start.x) / ray.direction.x;
    } else if ray.direction.x < 0.0 {
        step_x = -1;
        t_dx = 1.0 / -ray.direction.x;
        t_max_x = t_min + ((i.x) as f64 - start.x) / ray.direction.x;
    } else {
        step_x = 0;
        t_dx = t_max;
        t_max_x = t_max;
    }

    let step_y;
    let t_dy;
    let mut t_max_y;
    if ray.direction.y > 0.0 {
        step_y = 1;
        t_dy = 1.0 / ray.direction.y;
        t_max_y = t_min + ((i.y + 1) as f64 - start.y) / ray.direction.y;
    } else if ray.direction.y < 0.0 {
        step_y = -1;
        t_dy = 1.0 / -ray.direction.y;
        t_max_y = t_min + ((i.y) as f64 - start.y) / ray.direction.y;
    } else {
        step_y = 0;
        t_dy = t_max;
        t_max_y = t_max;
    }

    let step_z;
    let t_dz;
    let mut t_max_z;
    if ray.direction.z > 0.0 {
        step_z = 1;
        t_dz = 1.0 / ray.direction.z;
        t_max_z = t_min + ((i.z + 1) as f64 - start.z) / ray.direction.z;
    } else if ray.direction.z < 0.0 {
        step_z = -1;
        t_dz = 1.0 / -ray.direction.z;
        t_max_z = t_min + ((i.z) as f64 - start.z) / ray.direction.z;
    } else {
        step_z = 0;
        t_dz = t_max;
        t_max_z = t_max;
    }

    let mut countdown = view_settings.xray;
    let mut inside = t < 0.001 && grid[((i.x) + (i.z) * GRID_SIDE + (i.y) * GRID_SIDE_2) as usize];
    if inside {
        // get out first
        countdown += 1;
    }

    let mut backup_hit = None;

    while t < t_max {
        if grid[((i.x) + (i.z) * GRID_SIDE + (i.y) * GRID_SIDE_2) as usize] {
            if !inside {
                let p = ray.origin + t * ray.direction;
                let px = p.x - p.x.floor();
                let py = p.y - p.y.floor();
                let pz = p.z - p.z.floor();
                let (normal, u, v) = match axis {
                    Axis::X => (Vec3::new(-step_x as f64, 0.0, 0.0), pz, py),
                    Axis::Y => (Vec3::new(0.0, -step_y as f64, 0.0), px, pz),
                    Axis::Z => (Vec3::new(0.0, 0.0, -step_z as f64), px, py),
                };
                backup_hit = Some(TerrainHit::new(i, normal, t, u, v));
                if countdown == 0 {
                    return backup_hit;
                }
            }
            inside = true;
        } else {
            if inside && countdown > 0 {
                countdown -= 1;
            }
            inside = false;
        }

        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                i.x += step_x;
                t = t_max_x;
                t_max_x += t_dx;
                axis = Axis::X;
            } else {
                i.z += step_z;
                t = t_max_z;
                t_max_z += t_dz;
                axis = Axis::Z;
            }
        } else if t_max_y < t_max_z {
            i.y += step_y;
            t = t_max_y;
            t_max_y += t_dy;
            axis = Axis::Y;
        } else {
            i.z += step_z;
            t = t_max_z;
            t_max_z += t_dz;
            axis = Axis::Z;
        }
    }

    backup_hit
}

const VIEW_DISTANCE: f64 = 16.0;

fn ray_color(r: &Ray, _model: &Model, view_settings: &ViewSettings, grid: &Grid) -> Color {
    if let Some(TerrainHit {
        index: i,
        normal,
        t,
        u,
        v,
    }) = amanatides_woo(r, 0.0, f64::INFINITY, grid, view_settings)
    {
        let _diffuse = Color::new(
            (i.x % 4) as f64 / 4.0,
            (i.y % 4) as f64 / 4.0,
            (i.z % 4) as f64 / 4.0,
        );
        let diffuse = Color::new(u, v, 0.0);
        let _c = lerp(diffuse, normal, 0.5);
        let c = diffuse;
        return (1.0 - t / VIEW_DISTANCE) * c;
        // return Color::new(0.6, 0.4, 0.2);
    }

    lerp(
        Color::ONE,
        Color::new(0.5, 0.7, 0.9),
        0.5 * (-r.direction.normalized().y + 1.0),
    )
}

impl Renderer {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Renderer, Error> {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(width, height, surface_texture)?
        };

        Ok(Renderer {
            pixels,
            buf_width: width,
            buf_height: height,
        })
    }

    pub fn render(&mut self, world: &World) {
        let frame = self.pixels.get_frame();

        let wf = 1.0 / f64::from(self.buf_width);
        let hf = 1.0 / f64::from(self.buf_height);

        let mp = *world.view.mouse_pos.borrow();

        let mut grid = [false; GRID_SIDE_3 as usize];
        for (i, item) in grid.iter_mut().enumerate() {
            *item = i % 7 == 0;
        }

        // render:
        // 1. borrow resources immutably
        // 2. send to n threads for raycasting
        // 3. ask resources for hits

        let view_settings = &world.view.settings.borrow();

        // Draw
        frame.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
            let x = i % self.buf_width as usize;
            let y = i / self.buf_width as usize;

            let u = (x as f64) * wf;
            let v = 1.0 - (y as f64) * hf;

            // draw debug cursor
            if let Some(mp) = mp {
                if (x as i32 - (mp.x * self.buf_width as f64) as i32) == 0
                    && (y as i32 - (mp.y * self.buf_height as f64) as i32) == 0
                {
                    let rgba = [0xff, 0, 0, 0xff];
                    pixel.copy_from_slice(&rgba);
                    return;
                }
            }

            let cam = world.view.camera.borrow();
            let r1 = cam.get_ray(u, v);
            let r2 = cam.get_ray(u + 0.5 * wf, v);
            let r3 = cam.get_ray(u, v + 0.5 * hf);
            let r4 = cam.get_ray(u + 0.5 * wf, v + 0.5 * hf);
            let c1 = ray_color(&r1, &world.model, view_settings, &grid);
            let c2 = ray_color(&r2, &world.model, view_settings, &grid);
            let c3 = ray_color(&r3, &world.model, view_settings, &grid);
            let c4 = ray_color(&r4, &world.model, view_settings, &grid);
            let c = 0.25 * (c1 + c2 + c3 + c4);

            let rgba = [
                (255.999 * c.x) as u8,
                (255.999 * c.y) as u8,
                (255.999 * c.z) as u8,
                0xff,
            ];

            pixel.copy_from_slice(&rgba);
        });

        self.pixels.render().expect("Pixels failed to render");
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let w = f64::from(width);
        let h = f64::from(height);
        let f = (w.max(h) / 128.0) as u32;
        self.pixels.resize_surface(width, height);
        self.pixels.resize_buffer(width / f, height / f);
        self.buf_width = width / f;
        self.buf_height = height / f;
    }
}
