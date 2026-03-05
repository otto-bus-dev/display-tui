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

    pub fn to_hyprland(&self) -> String {
        match self {
            Rotation::Normal => "".to_string(),
            Rotation::Deg90 => "transform, 1".to_string(),
            Rotation::Deg180 => "transform, 2".to_string(),
            Rotation::Deg270 => "transform, 3".to_string(),
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
