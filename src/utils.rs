#[derive(Default,Debug, Clone, Copy, PartialEq)]
pub enum TUIMode {
    #[default]
    View,
    Move,
    Resolution,
    Scale
}

pub struct ScaleValue {
    pub name: &'static str,
    pub value: f32,
}
impl ScaleValue {
    pub fn new(name: &'static str, value: f32) -> Self {
        ScaleValue { name, value }
    }
    pub fn table() -> Vec<Self> {
        vec![
            ScaleValue::new("50%", 0.5),
            ScaleValue::new("66%", 0.6),
            ScaleValue::new("75%", 0.75),
            ScaleValue::new("80%", 0.8),
            ScaleValue::new("100%", 1.0),
            ScaleValue::new("125%", 1.25),
            ScaleValue::new("160%", 1.6),
            ScaleValue::new("175%", 1.75),
            ScaleValue::new("200%", 2.0),
        ]
    }
}
