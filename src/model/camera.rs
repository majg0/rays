use std::f64::consts::PI;

use crate::*;

pub struct Camera {
    pub origin: Point3,
    pub lookat: Point3,
    pub up: Vec3,
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub dirty: bool,

    // derivatives
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, lookat: Point3, up: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let mut c = Camera {
            origin,
            lookat,
            up,
            vfov,
            aspect_ratio,
            lower_left_corner: Point3::ZERO,
            horizontal: Vec3::ZERO,
            vertical: Vec3::ZERO,
            dirty: true,
        };
        c.refresh();
        c
    }

    pub fn refresh(&mut self) {
        if !self.dirty {
            return;
        }

        let theta = self.vfov * PI / 180.0;
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.origin - self.lookat).normalized();
        let u = self.up.cross(w).normalized();
        let v = w.cross(u);

        self.horizontal = viewport_width * u;
        self.vertical = viewport_height * v;
        self.lower_left_corner = self.origin - 0.5 * self.horizontal - 0.5 * self.vertical - w;
        self.dirty = false;
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            (self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
                .normalized(),
        )
    }
}
