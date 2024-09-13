use std::collections::HashMap;

use schemars::JsonSchema;
use sdlc_cp_api_macro::RegisterSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::policy::Vulnerability;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, RegisterSchema, ToSchema, PartialEq)]
pub enum SDLCPhase {
    Development,
    Source,
    Build,
    Package,
    Deploy,
    Runtime,
    Custom(String),
}

impl SDLCPhase {
    pub fn name(&self) -> &str {
        match self {
            SDLCPhase::Development => "Development",
            SDLCPhase::Source => "Source",
            SDLCPhase::Build => "Build",
            SDLCPhase::Package => "Package",
            SDLCPhase::Deploy => "Deploy",
            SDLCPhase::Runtime => "Runtime",
            SDLCPhase::Custom(custom_name) => custom_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, RegisterSchema)]
pub struct PhaseDetails {
    pub development_details: Option<DevelopmentDetails>,
    pub source_details: Option<SourceDetails>,
    pub build_details: Option<BuildDetails>,
    pub package_details: Option<PackageDetails>,
    pub deploy_details: Option<DeployDetails>,
    pub runtime_details: Option<RuntimeDetails>,
    pub custom_details: HashMap<String, Option<serde_json::Value>>,
}

impl PhaseDetails {
    pub fn new() -> Self {
        PhaseDetails {
            development_details: None,
            source_details: None,
            build_details: None,
            package_details: None,
            deploy_details: None,
            runtime_details: None,
            custom_details: HashMap::new(),
        }
    }
}


#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub struct DevelopmentDetails {
    pub feature_list: Vec<String>,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub struct SourceDetails {
    pub commit_hash: String,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub struct BuildDetails {
    pub build_id: String,
    pub build_timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub struct PackageDetails {
    pub artifact_hash: String,
    pub artifact_url: String,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub struct DeployDetails {
    pub deployment_id: String,
    pub environment: String,
}


#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema, Serialize, Deserialize)]
pub struct RuntimeDetails {
    pub runtime_id: String,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub vulnerabilities: Vec<Vulnerability>,
}