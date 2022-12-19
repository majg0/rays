use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Model {
    pub terrain: Terrain,
    pub entity: Allocator,
    pub position: ComponentStorage<Vec3>,
    pub velocity: ComponentStorage<Vec3>,
    pub sphere: ComponentStorage<Sphere>,
}

impl Model {
    pub fn new(terrain: Terrain) -> Model {
        Model {
            terrain,
            ..Model::default()
        }
    }

    pub fn lerp(&self, rhs: &Model, t: f64) -> Model {
        let m = self.clone();

        {
            let mut p1 = m.position.write();
            let p2 = rhs.position.read();
            for e in iterate(&mut [&m.entity, &m.position, &rhs.position]) {
                p1[e] = lerp(p1[e], p2[e], t);
            }
        }

        {
            let mut p1 = m.velocity.write();
            let p2 = rhs.velocity.read();
            for e in iterate(&mut [&m.entity, &m.velocity, &rhs.velocity]) {
                p1[e] = lerp(p1[e], p2[e], t);
            }
        }

        m
    }
}
