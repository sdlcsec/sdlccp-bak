use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sdlc_cp_api_macro::RegisterSchema;

use super::policy::VulnerabilityLevel;

pub trait ReleaseState: Clone {
    fn name() -> &'static str;
    fn new() -> Self;
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Draft;

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct InProgress {
    pub started_by: String,
    pub started_at: DateTime<Utc>,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct PolicyCheckPending {
    pub initiated_at: DateTime<Utc>,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct PolicyCheckFailed {
    pub failed_at: DateTime<Utc>,
    pub reason: String,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Releasable {
    pub approved_at: DateTime<Utc>,
    pub approved_by: String,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Released {
    pub release_time: DateTime<Utc>,
    pub release_notes: String,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Deployed {
    pub deployment_time: DateTime<Utc>,
    pub environment: String,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Vulnerability {
    pub id: String,
    pub severity: VulnerabilityLevel,
    pub description: String,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Revoked {
    pub revocation_time: DateTime<Utc>,
    pub reason: String,
}

impl ReleaseState for Draft {
    fn name() -> &'static str { "Draft" }
    fn new() -> Self { Draft }
}

impl ReleaseState for InProgress {
    fn name() -> &'static str { "InProgress" }
    fn new() -> Self {
        InProgress {
            started_by: String::new(),
            started_at: Utc::now(),
        }
    }
}

impl InProgress {
    pub fn new(started_by: String) -> Self {
        InProgress {
            started_by,
            started_at: Utc::now(),
        }
    }
}

impl ReleaseState for PolicyCheckPending {
    fn name() -> &'static str { "PolicyCheckPending" }
    fn new() -> Self {
        PolicyCheckPending {
            initiated_at: Utc::now(),
        }
    }
}

impl ReleaseState for PolicyCheckFailed {
    fn name() -> &'static str { "PolicyCheckFailed" }
    fn new() -> Self {
        PolicyCheckFailed {
            failed_at: Utc::now(),
            reason: String::new(),
        }
    }
}

impl PolicyCheckFailed {
    pub fn new(reason: String) -> Self {
        PolicyCheckFailed {
            failed_at: Utc::now(),
            reason,
        }
    }
}

impl ReleaseState for Releasable {
    fn name() -> &'static str { "Releasable" }
    fn new() -> Self {
        Releasable {
            approved_at: Utc::now(),
            approved_by: String::new(),
        }
    }
}

impl Releasable {
    pub fn new_with_approver(approved_by: String) -> Self {
        Releasable {
            approved_at: Utc::now(),
            approved_by,
        }
    }
}

impl ReleaseState for Released {
    fn name() -> &'static str { "Released" }
    fn new() -> Self {
        Released {
            release_time: Utc::now(),
            release_notes: String::new(),
        }
    }
}

impl Released {
    pub fn new(release_notes: String) -> Self {
        Released {
            release_time: Utc::now(),
            release_notes,
        }
    }
}

impl ReleaseState for Deployed {
    fn name() -> &'static str { "Deployed" }
    fn new() -> Self {
        Deployed {
            deployment_time: Utc::now(),
            environment: String::new(),
            vulnerabilities: Vec::new(),
        }
    }
}

impl Deployed {
    pub fn new(environment: String) -> Self {
        Deployed {
            deployment_time: Utc::now(),
            environment,
            vulnerabilities: Vec::new(),
        }
    }
}

impl ReleaseState for Revoked {
    fn name() -> &'static str { "Revoked" }
    fn new() -> Self {
        Revoked {
            revocation_time: Utc::now(),
            reason: String::new(),
        }
    }
}

impl Revoked {
    pub fn new(reason: String) -> Self {
        Revoked {
            revocation_time: Utc::now(),
            reason,
        }
    }
}