use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sdlccp_api_macro::RegisterSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents the state of a release within a phase.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, RegisterSchema, ToSchema, PartialEq)]
pub enum ReleaseState {
    Draft,
    InProgress { started_by: String, started_at: DateTime<Utc> },
    Releasable { approved_by: String, approved_at: DateTime<Utc> },
    Released { release_notes: String, release_time: DateTime<Utc> },
    Deployed { environment: String, deployment_time: DateTime<Utc> },
    Revoked { reason: String, revocation_time: DateTime<Utc> },
    Custom(String),
}

impl ReleaseState {
    pub fn name(&self) -> &str {
        match self {
            ReleaseState::Draft => "Draft",
            ReleaseState::InProgress { .. } => "InProgress",
            ReleaseState::Releasable { .. } => "Releasable",
            ReleaseState::Released { .. } => "Released",
            ReleaseState::Deployed { .. } => "Deployed",
            ReleaseState::Revoked { .. } => "Revoked",
            ReleaseState::Custom(name) => name,
        }
    }
}