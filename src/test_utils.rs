#[cfg(test)]
pub mod tests {
    use crate::monitor::{Monitor, Resolution, Position};
    pub fn test_monitors() -> Vec<Monitor> {
        vec![
            Monitor {
                name: "Monitor 1".to_string(),
                description: Some("Description 1".to_string()),
                make:Some("Some".to_string()),
                model:Some("Description".to_string()),
                serial:Some("1".to_string()),
                enabled: true,
                modes: vec![
                    Resolution { width: 1920, height: 1080, refresh:60.0, preferred: true ,current: true},
                    Resolution { width: 1280, height: 720 , refresh:60.0, preferred: false,current: false},
                ],
                position: Some(Position { x: 0, y: 0 }),
                scale: Some(1.0),
                transform: None,
                saved_position: None,
                saved_scale: None,
            },
            Monitor {
                name: "Monitor 2".to_string(),
                description: Some("Description 2".to_string()),
                make:Some("Some".to_string()),
                model:Some("Description".to_string()),
                serial:Some("2".to_string()),
                enabled: false,
                modes: vec![
                    Resolution { width: 1920, height: 1080 , refresh:60.0, preferred: false, current: false },
                    Resolution { width: 1280, height: 720 , refresh:60.0, preferred: true, current: true},
                ],
                position: Some(Position { x: 1920, y: 0 }),
                scale: Some(1.25),
                transform: None,
                saved_position: None,
                saved_scale: None,
            },
        ]
    }
}   
