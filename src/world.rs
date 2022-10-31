use crate::*;

pub struct World {
    pub model: Model,
    pub prev_model: Model,
    pub view: View,
}

impl World {
    pub fn new(view: View) -> World {
        World {
            model: Model::default(),
            prev_model: Model::default(),
            view,
        }
    }
}
