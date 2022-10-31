use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    // TODO: remove
    pub eid: u32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, eid: u32) -> Sphere {
        Sphere {
            center,
            radius,
            eid,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _: &ViewSettings) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (p - self.center) / self.radius;
        let front_face = r.direction.dot(normal) < 0.0;
        Some(HitRecord::new(p, normal, root, front_face, self.eid))
    }
}
