use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub enum SessionType {
    Application,
    Device,
    System,
    #[default]
    Unknown,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub enum SessionDirection {
    Render,
    Capture,
    #[default]
    Unknown,
}
