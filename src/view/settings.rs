pub struct ViewSettings {
    pub xray: usize,
    pub view_distance: f64,
}

impl ViewSettings {
    pub fn new() -> ViewSettings {
        ViewSettings {
            xray: 0,
            view_distance: 16.0,
        }
    }
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self::new()
    }
}
