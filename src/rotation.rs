use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Rotation {
    Normal,
    Deg90,
    Deg180,
    Deg270,
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::Normal
    }
}

impl Rotation {
    pub fn from_transform(transform: &Option<String>) -> Self {
        match transform.as_deref() {
            Some("90") => Rotation::Deg90,
            Some("180") => Rotation::Deg180,
            Some("270") => Rotation::Deg270,
            _ => Rotation::Normal,
        }
    }

    pub fn to_transform(&self) -> &str {
        match self {
            Rotation::Normal => "normal",
            Rotation::Deg90 => "90",
            Rotation::Deg180 => "180",
            Rotation::Deg270 => "270",
        }
    }

    pub fn to_hyprland(&self) -> i32 {
        match self {
            Rotation::Normal => 0,
            Rotation::Deg90 => 1,
            Rotation::Deg180 => 2,
            Rotation::Deg270 => 3,
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Rotation::Normal => Rotation::Deg90,
            Rotation::Deg90 => Rotation::Deg180,
            Rotation::Deg180 => Rotation::Deg270,
            Rotation::Deg270 => Rotation::Normal,
        }
    }
}
