use schemars::JsonSchema;
use sdlc_cp_api_macro::RegisterSchema;

pub trait SDLCPhase {
    fn name() -> &'static str;
}

#[derive(Debug, Clone, JsonSchema)]
pub struct Development;

#[derive(Debug, Clone, JsonSchema)]
pub struct Source;

#[derive(Debug, Clone, JsonSchema)]
pub struct Build;

#[derive(Debug, Clone, JsonSchema)]
pub struct Package;

#[derive(Debug, Clone, JsonSchema)]
pub struct Deploy;

#[derive(Debug, Clone, JsonSchema,)]
pub struct Runtime;

impl SDLCPhase for Development { fn name() -> &'static str { "Development" } }
impl SDLCPhase for Source { fn name() -> &'static str { "Source" } }
impl SDLCPhase for Build { fn name() -> &'static str { "Build" } }
impl SDLCPhase for Package { fn name() -> &'static str { "Package" } }
impl SDLCPhase for Deploy { fn name() -> &'static str { "Deploy" } }
impl SDLCPhase for Runtime { fn name() -> &'static str { "Runtime" } }

pub struct DevelopmentDetails {
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub struct SourceDetails {
    pub commit_hash: String,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub struct BuildDetails {
    pub build_id: String,
    pub build_timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub struct PackageDetails {
    pub artifact_hash: String,
    pub artifact_url: String,
}

#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub struct DeployDetails {
    pub deployment_id: String,
    pub environment: String,
}


#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub struct RuntimeDetails {
    pub runtime_id: String,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
}