use super::sdlc_component::SDLCComponent;
use super::phase::{BuildDetails, DeployDetails, DevelopmentDetails, PackageDetails, PhaseDetails, SDLCPhase, SourceDetails};
use super::state::ReleaseState;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sdlccp_api_macro::RegisterSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, JsonSchema, ToSchema, Serialize, Deserialize, RegisterSchema)]
pub struct SDLCRelease {
    pub id: Uuid,
    pub component: SDLCComponent,
    pub version: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub commit_hash: Option<String>,
    pub dependencies: Vec<Uuid>, // IDs of dependent releases
    pub phase_attestations: HashMap<String, Uuid>, // Todo: this is currently Phase name to attestation ID. Should this be a HashMap<Phase, Uuid>?
    #[schema(value_type = Object)]
    pub state: ReleaseState,
    pub phase: SDLCPhase,
    pub phase_details: Option<PhaseDetails>,
}

impl SDLCRelease {
    /// Creates a new SDLCRelease in the Development phase with Draft state.
    pub fn new(component: SDLCComponent, version: String, created_by: String) -> Self {
        SDLCRelease {
            id: Uuid::new_v4(),
            component,
            version,
            created_by,
            created_at: Utc::now(),
            commit_hash: None,
            dependencies: Vec::new(),
            phase_attestations: HashMap::new(),
            phase: SDLCPhase::Development,
            state: ReleaseState::Draft,
            phase_details: Some(PhaseDetails::new()),
        }
    }

    // Helper methods
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn phase_name(&self) -> &str {
        self.phase.name()
    }

    pub fn state_name(&self) -> &str {
        self.state.name()
    }

    pub fn component_name(&self) -> &str {
        &self.component.name()
    }

    pub fn add_dependency(&mut self, dependency_id: Uuid) {
        self.dependencies.push(dependency_id);
    }

    pub fn add_phase_attestation(&mut self, phase_name: String, attestation_id: Uuid) {
        self.phase_attestations.insert(phase_name, attestation_id);
    }

    /// Starts the Development phase.
    pub fn start_development(&mut self, started_by: String, feature_list: Vec<String>) -> Result<(), String> {
        if self.phase == SDLCPhase::Development && matches!(self.state, ReleaseState::Draft) {
            self.state = ReleaseState::InProgress {
                started_by,
                started_at: Utc::now(),
            };
            if let Some(details) = &mut self.phase_details {
                details.development_details = Some(DevelopmentDetails { feature_list });
            }
            Ok(())
        } else {
            Err("Cannot start development in the current phase and state.".to_string())
        }
    }

    /// Completes the Development phase.
    pub fn complete_development(&mut self) -> Result<(), String> {
        if self.phase == SDLCPhase::Development && matches!(self.state, ReleaseState::InProgress { .. }) {
            self.phase = SDLCPhase::Source;
            self.state = ReleaseState::Draft;
            if let Some(details) = &mut self.phase_details {
                details.development_details = None;
                details.source_details = None;
            }
            Ok(())
        } else {
            Err("Cannot complete development in the current phase and state.".to_string())
        }
    }

    /// Starts the Source Review phase.
    pub fn start_source_review(&mut self, started_by: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Source && matches!(self.state, ReleaseState::Draft) {
            self.state = ReleaseState::InProgress {
                started_by,
                started_at: Utc::now(),
            };
            Ok(())
        } else {
            Err("Cannot start source review in the current phase and state.".to_string())
        }
    }

    /// Completes the Source Review phase.
    pub fn complete_source_review(&mut self, commit_hash: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Source && matches!(self.state, ReleaseState::InProgress { .. }) {
            self.commit_hash = Some(commit_hash.clone());
            if let Some(details) = &mut self.phase_details {
                details.source_details = Some(SourceDetails { commit_hash });
            }
            self.phase = SDLCPhase::Build;
            self.state = ReleaseState::Draft;
            Ok(())
        } else {
            Err("Cannot complete source review in the current phase and state.".to_string())
        }
    }

    // Implement other phase transitions similarly

    /// Starts the Build phase.
    pub fn start_build(&mut self, started_by: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Build && matches!(self.state, ReleaseState::Draft) {
            self.state = ReleaseState::InProgress {
                started_by,
                started_at: Utc::now(),
            };
            Ok(())
        } else {
            Err("Cannot start build in the current phase and state.".to_string())
        }
    }

    /// Completes the Build phase.
    pub fn complete_build(&mut self, build_id: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Build && matches!(self.state, ReleaseState::InProgress { .. }) {
            if let Some(details) = &mut self.phase_details {
                details.build_details = Some(BuildDetails {
                    build_id,
                    build_timestamp: Utc::now(),
                });
            }
            self.phase = SDLCPhase::Package;
            self.state = ReleaseState::Draft;
            Ok(())
        } else {
            Err("Cannot complete build in the current phase and state.".to_string())
        }
    }

    /// Starts the Packaging phase.
    pub fn start_packaging(&mut self, started_by: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Package && matches!(self.state, ReleaseState::Draft) {
            self.state = ReleaseState::InProgress {
                started_by,
                started_at: Utc::now(),
            };
            Ok(())
        } else {
            Err("Cannot start packaging in the current phase and state.".to_string())
        }
    }

    /// Completes the Packaging phase.
    pub fn complete_packaging(&mut self, artifact_hash: String, artifact_url: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Package && matches!(self.state, ReleaseState::InProgress { .. }) {
            if let Some(details) = &mut self.phase_details {
                details.package_details = Some(PackageDetails {
                    artifact_hash,
                    artifact_url,
                });
            }
            self.phase = SDLCPhase::Deploy;
            self.state = ReleaseState::Releasable {
                approved_by: "Auto-Approved".to_string(),
                approved_at: Utc::now(),
            };
            Ok(())
        } else {
            Err("Cannot complete packaging in the current phase and state.".to_string())
        }
    }

    /// Releases the package.
    pub fn release(&mut self, release_notes: String) -> Result<(), String> {
        if matches!(self.state, ReleaseState::Releasable { .. }) {
            self.state = ReleaseState::Released {
                release_notes,
                release_time: Utc::now(),
            };
            Ok(())
        } else {
            Err("Cannot release in the current state.".to_string())
        }
    }

    /// Starts the Deployment phase.
    pub fn start_deployment(&mut self, environment: String) -> Result<(), String> {
        if self.phase == SDLCPhase::Deploy && matches!(self.state, ReleaseState::Released { .. }) {
            self.state = ReleaseState::InProgress {
                started_by: "Deployment System".to_string(),
                started_at: Utc::now(),
            };
            if let Some(details) = &mut self.phase_details {
                details.deploy_details = Some(DeployDetails {
                    deployment_id: Uuid::new_v4().to_string(),
                    environment: environment.clone(),
                });
            }
            Ok(())
        } else {
            Err("Cannot start deployment in the current phase and state.".to_string())
        }
    }

    /// Completes the Deployment phase.
    pub fn complete_deployment(&mut self) -> Result<(), String> {
        if self.phase == SDLCPhase::Deploy && matches!(self.state, ReleaseState::InProgress { .. }) {
            self.phase = SDLCPhase::Runtime;
            if let Some(details) = &mut self.phase_details {
                if let Some(deploy_details) = &details.deploy_details {
                    self.state = ReleaseState::Deployed {
                        environment: deploy_details.environment.clone(),
                        deployment_time: Utc::now(),
                    };
                } else {
                    return Err("Deployment details missing.".to_string());
                }
            } else {
                return Err("Phase details missing.".to_string());
            }
            Ok(())
        } else {
            Err("Cannot complete deployment in the current phase and state.".to_string())
        }
    }

    /// Revokes the release.
    pub fn revoke(&mut self, reason: String) -> Result<(), String> {
        if matches!(self.state, ReleaseState::Deployed { .. }) {
            self.state = ReleaseState::Revoked {
                reason,
                revocation_time: Utc::now(),
            };
            Ok(())
        } else {
            Err("Cannot revoke in the current state.".to_string())
        }
    }

    /// Validates the current phase and state.
    pub fn validate(&self) -> Result<(), String> {
        match (&self.phase, &self.state) {
            (SDLCPhase::Development, ReleaseState::Draft)
            | (SDLCPhase::Development, ReleaseState::InProgress { .. }) => Ok(()),
            (SDLCPhase::Source, ReleaseState::Draft)
            | (SDLCPhase::Source, ReleaseState::InProgress { .. }) => Ok(()),
            // Add other valid combinations as needed
            _ => Err("Invalid phase and state combination.".to_string()),
        }
    }
}

/* 
impl<Phase: SDLCPhase, State: ReleaseState> SDLCRelease<Phase, State> {
    pub fn new(component: SDLCComponent, version: String, created_by: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            component,
            version,
            created_by,
            created_at: Utc::now(),
            commit_hash: None,
            dependencies: Vec::new(),
            phase_attestations: HashMap::new(),
            state_info: State::new(),
            _phase: PhantomData,
        }
    }

    pub fn set_commit_hash(&mut self, commit_hash: String) {
        self.commit_hash = Some(commit_hash);
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn phase_name(&self) -> &'static str {
        Phase::name()
    }

    pub fn state_name(&self) -> &'static str {
        State::name()
    }

    pub fn component_name(&self) -> &str {
        match &self.component {
            SDLCComponent::Project(project) => &project.name,
            SDLCComponent::Unmanaged(unmanaged) => &unmanaged.name,
        }
    }

    pub fn component_type(&self) -> &str {
        match self.component {
            SDLCComponent::Project(_) => "Project",
            SDLCComponent::Unmanaged(_) => "Unmanaged",
        }
    }

    pub fn add_dependency(&mut self, dependency_id: Uuid) {
        self.dependencies.push(dependency_id);
    }

    pub fn add_phase_attestation(&mut self, phase: String, attestation_id: Uuid) {
        self.phase_attestations.insert(phase, attestation_id);
    }
}

// Phase transition implementations

impl SDLCRelease<phase::Development, state::Draft> {
    pub fn start_development(self, started_by: String) -> SDLCRelease<phase::Development, state::InProgress> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::InProgress::new(started_by),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Development, state::InProgress> {
    pub fn complete_development(self, _details: phase::DevelopmentDetails) -> SDLCRelease<phase::Source, state::Draft> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Draft::new(),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Source, state::Draft> {
    pub fn start_source_review(self, reviewer: String) -> SDLCRelease<phase::Source, state::InProgress> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::InProgress::new(reviewer),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Source, state::InProgress> {
    pub fn complete_source_review(self, details: phase::SourceDetails) -> SDLCRelease<phase::Build, state::Draft> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: Some(details.commit_hash),
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Draft::new(),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Build, state::Draft> {
    pub fn start_build(self, builder: String) -> SDLCRelease<phase::Build, state::InProgress> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::InProgress::new(builder),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Build, state::InProgress> {
    pub fn complete_build(self, _details: phase::BuildDetails) -> SDLCRelease<phase::Package, state::Draft> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Draft::new(),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Package, state::Draft> {
    pub fn start_packaging(self, packager: String) -> SDLCRelease<phase::Package, state::InProgress> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::InProgress::new(packager),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Package, state::InProgress> {
    pub fn complete_packaging(self, _details: phase::PackageDetails) -> SDLCRelease<phase::Package, state::Releasable> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Releasable::new(),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Package, state::Releasable> {
    pub fn release(self, release_notes: String) -> SDLCRelease<phase::Deploy, state::Released> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Released::new(release_notes),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Deploy, state::Released> {
    pub fn start_deployment(self, environment: String) -> SDLCRelease<phase::Deploy, state::InProgress> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::InProgress::new(environment),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Deploy, state::InProgress> {
    pub fn complete_deployment(self, details: phase::DeployDetails) -> SDLCRelease<phase::Runtime, state::Deployed> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Deployed::new(details.environment),
            _phase: PhantomData,
        }
    }
}

impl SDLCRelease<phase::Runtime, state::Deployed> {
    pub fn revoke(self, reason: String) -> SDLCRelease<phase::Runtime, state::Revoked> {
        SDLCRelease {
            id: self.id,
            component: self.component,
            version: self.version,
            created_by: self.created_by,
            created_at: self.created_at,
            commit_hash: self.commit_hash,
            dependencies: self.dependencies,
            phase_attestations: self.phase_attestations,
            state_info: state::Revoked::new(reason),
            _phase: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::phase;
    use crate::model::sdlc_component::Project;
    use crate::model::state;

    #[test]
    fn test_sdlc_release_lifecycle() {
        let project = Project {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            repository_url: Some("https://github.com/test/project".to_string()),
            owner: Some("Test Owner".to_string()),
            components: Vec::new(),
        };
        let component = SDLCComponent::Project(project);
        
        // Initialize a new release
        let release = SDLCRelease::<phase::Development, state::Draft>::new(
            component,
            "1.0.0".to_string(),
            "developer1".to_string()
        );

        assert_eq!(release.phase_name(), "Development");
        assert_eq!(release.state_name(), "Draft");
        assert_eq!(release.created_by, "developer1");

        // Start development
        let release = release.start_development("developer2".to_string());
        assert_eq!(release.phase_name(), "Development");
        assert_eq!(release.state_name(), "InProgress");
        assert_eq!(release.state_info.started_by, "developer2");

        // Complete development
        let release = release.complete_development(phase::DevelopmentDetails {});
        assert_eq!(release.phase_name(), "Source");
        assert_eq!(release.state_name(), "Draft");

        // Start source review
        let release = release.start_source_review("reviewer1".to_string());
        assert_eq!(release.phase_name(), "Source");
        assert_eq!(release.state_name(), "InProgress");
        assert_eq!(release.state_info.started_by, "reviewer1");

        // Complete source review
        let release = release.complete_source_review(phase::SourceDetails {
            commit_hash: "abcdef123456".to_string(),
        });
        assert_eq!(release.phase_name(), "Build");
        assert_eq!(release.state_name(), "Draft");
        assert_eq!(release.commit_hash, Some("abcdef123456".to_string()));

        // Continue with other phases...
        // (Build, Package, Deploy, Runtime)

        // Final check
        assert_eq!(release.version(), "1.0.0");
        assert_eq!(release.component_name(), "Test Project");
        assert_eq!(release.component_type(), "Project");
    }

    // Add more tests as needed
}

*/