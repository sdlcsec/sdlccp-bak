use super::sdlc_component::SDLCComponent;
use super::phase::{self, SDLCPhase};
use super::state::{self, ReleaseState};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, JsonSchema, ToSchema, Serialize, Deserialize)]
#[serde(bound = "State: Serialize + for<'des> Deserialize<'des>")]
pub struct SDLCRelease<Phase: SDLCPhase, State: ReleaseState> {
    pub id: Uuid,
    pub component: SDLCComponent,
    pub version: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub commit_hash: Option<String>,
    pub dependencies: Vec<Uuid>, // IDs of dependent releases
    pub phase_attestations: HashMap<String, Uuid>, // Todo: this is currently Phase name to attestation ID. Should this be a HashMap<Phase, Uuid>?
    #[schema(value_type = Object)]
    pub state_info: State,
    #[serde(skip)]
    pub(crate) _phase: PhantomData<Phase>,
}

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

