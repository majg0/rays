use crate::*;

pub struct World {
    pub model: Model,
    pub prev_model: Model,
    pub view: View,
}

impl World {
    pub fn new(model: Model, view: View) -> World {
        let prev_model = model.clone();
        World {
            model,
            prev_model,
            view,
        }
    }
}
