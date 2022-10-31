use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub eid: u32,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool, eid: u32) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            eid,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, settings: &ViewSettings) -> Option<HitRecord>;
}
