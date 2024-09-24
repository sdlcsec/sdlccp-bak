use std::collections::HashMap;

use schemars::JsonSchema;
use sdlccp_api_macro::RegisterSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub enum SDLCComponent {
    Project(Project),   
    Unmanaged(Unmanaged),
}

#[derive(Debug, Clone, JsonSchema, ToSchema, Serialize, Deserialize)]
pub struct Unmanaged {
    pub id: Uuid,
    pub name: String,
    pub repository_url: Option<String>,
    pub package_url: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, JsonSchema, ToSchema, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub repository_url: Option<String>,
    pub owner: Option<String>,
    pub components: Vec<Uuid>, // References to other SDLCComponents
}

impl SDLCComponent {
    pub fn name(&self) -> &str {
        match self {
            SDLCComponent::Project(p) => p.name.as_str(),
            SDLCComponent::Unmanaged(u) => u.name.as_str(),
        }
    }
}