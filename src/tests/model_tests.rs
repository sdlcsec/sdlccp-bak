use crate::model::*;
use attestation::{Subject, SubjectType};
use chrono::{Duration as ChronoDuration, Utc};
use policy::{Policy, PolicyRule, VulnerabilityLevel};
use sdlc_component::{Project, SDLCComponent, Unmanaged};
use state::Vulnerability;
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_sdlc_release_lifecycle_project() {
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
    assert_eq!(release.component_name(), "Test Project");
    assert_eq!(release.component_type(), "Project");

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

    // Start build
    let release = release.start_build("builder1".to_string());
    assert_eq!(release.phase_name(), "Build");
    assert_eq!(release.state_name(), "InProgress");
    assert_eq!(release.state_info.started_by, "builder1");

    // Complete build
    let release = release.complete_build(phase::BuildDetails {
        build_id: "build123".to_string(),
        build_timestamp: Utc::now(),
    });
    assert_eq!(release.phase_name(), "Package");
    assert_eq!(release.state_name(), "Draft");

    // Start packaging
    let release = release.start_packaging("packager1".to_string());
    assert_eq!(release.phase_name(), "Package");
    assert_eq!(release.state_name(), "InProgress");
    assert_eq!(release.state_info.started_by, "packager1");

    // Complete packaging
    let release = release.complete_packaging(phase::PackageDetails {
        artifact_hash: "123abc456def".to_string(),
        artifact_url: "https://example.com/artifacts/project1-1.0.0.tar.gz".to_string(),
    });
    assert_eq!(release.phase_name(), "Package");
    assert_eq!(release.state_name(), "Releasable");

    // Release
    let release = release.release("Version 1.0.0 release notes".to_string());
    assert_eq!(release.phase_name(), "Deploy");
    assert_eq!(release.state_name(), "Released");
    assert_eq!(release.state_info.release_notes, "Version 1.0.0 release notes");

    // Start deployment
    let release = release.start_deployment("production".to_string());
    assert_eq!(release.phase_name(), "Deploy");
    assert_eq!(release.state_name(), "InProgress");
    assert_eq!(release.state_info.started_by, "production");

    // Complete deployment
    let release = release.complete_deployment(phase::DeployDetails {
        deployment_id: "deploy123".to_string(),
        environment: "production".to_string(),
    });
    assert_eq!(release.phase_name(), "Runtime");
    assert_eq!(release.state_name(), "Deployed");
    assert_eq!(release.state_info.environment, "production");

    // Revoke
    let release = release.revoke("Critical bug found".to_string());
    assert_eq!(release.phase_name(), "Runtime");
    assert_eq!(release.state_name(), "Revoked");
    assert_eq!(release.state_info.reason, "Critical bug found");
}

#[test]
fn test_sdlc_release_lifecycle_unmanaged() {
    let unmanaged = Unmanaged {
        id: Uuid::new_v4(),
        name: "Unmanaged Component".to_string(),
        repository_url: None,
        package_url: Some("https://example.com/packages/unmanaged-1.0.0.tar.gz".to_string()),
        metadata: HashMap::new(),
    };
    let component = SDLCComponent::Unmanaged(unmanaged);
    
    // Initialize a new release
    let release = SDLCRelease::<phase::Development, state::Draft>::new(
        component,
        "1.0.0".to_string(),
        "integrator1".to_string()
    );

    assert_eq!(release.phase_name(), "Development");
    assert_eq!(release.state_name(), "Draft");
    assert_eq!(release.created_by, "integrator1");
    assert_eq!(release.component_name(), "Unmanaged Component");
    assert_eq!(release.component_type(), "Unmanaged");

    // For unmanaged components, we might skip some phases or handle them differently
    // Here's an example of a simplified lifecycle:

    // Start integration (using development phase)
    let release = release.start_development("integrator2".to_string());
    assert_eq!(release.phase_name(), "Development");
    assert_eq!(release.state_name(), "InProgress");
    assert_eq!(release.state_info.started_by, "integrator2");

    // Complete integration
    let release = release.complete_development(phase::DevelopmentDetails {});
    assert_eq!(release.phase_name(), "Source");
    assert_eq!(release.state_name(), "Draft");

    // Skip source review for unmanaged component
    let release = SDLCRelease::<phase::Build, state::Draft> {
        id: release.id,
        component: release.component,
        version: release.version,
        created_by: release.created_by,
        created_at: release.created_at,
        commit_hash: release.commit_hash,
        dependencies: release.dependencies,
        phase_attestations: release.phase_attestations,
        state_info: state::Draft::new(),
        _phase: std::marker::PhantomData,
    };

    // Start build (which might be just a verification process for unmanaged components)
    let release = release.start_build("verifier1".to_string());
    assert_eq!(release.phase_name(), "Build");
    assert_eq!(release.state_name(), "InProgress");
    assert_eq!(release.state_info.started_by, "verifier1");

    // Complete build/verification
    let release = release.complete_build(phase::BuildDetails {
        build_id: "verify123".to_string(),
        build_timestamp: Utc::now(),
    });
    assert_eq!(release.phase_name(), "Package");
    assert_eq!(release.state_name(), "Draft");

    // For unmanaged components, packaging might be skipped as it's already packaged
    // Move directly to Releasable state
    let release = SDLCRelease::<phase::Package, state::Releasable> {
        id: release.id,
        component: release.component,
        version: release.version,
        created_by: release.created_by,
        created_at: release.created_at,
        commit_hash: release.commit_hash,
        dependencies: release.dependencies,
        phase_attestations: release.phase_attestations,
        state_info: state::Releasable::new(),
        _phase: std::marker::PhantomData,
    };

    assert_eq!(release.phase_name(), "Package");
    assert_eq!(release.state_name(), "Releasable");

    // Release
    let release = release.release("Unmanaged Component 1.0.0 integrated".to_string());
    assert_eq!(release.phase_name(), "Deploy");
    assert_eq!(release.state_name(), "Released");
    assert_eq!(release.state_info.release_notes, "Unmanaged Component 1.0.0 integrated");

    // Deployment and runtime phases would proceed as normal
    // ...
}

#[test]
fn test_policy_checks() {
    // Create a policy
    let mut policy = Policy::new("Security Policy".to_string(), vec!["Source".to_string(), "Build".to_string()]);
    
    // Add some rules
    use std::time::Duration;
    
    policy.add_rule(PolicyRule::MaxAge(Duration::from_secs(7 * 24 * 60 * 60)));
    policy.add_rule(PolicyRule::ApprovedIdentities(vec!["trusted_developer".to_string()]));
    policy.add_rule(PolicyRule::VulnerabilityThreshold(VulnerabilityLevel::High, 0));

    // Create an attestation
    let mut attestation = Attestation::new(
        Subject {
            type_: SubjectType::Artifact,
            name: "app-1.0.0.jar".to_string(),
            digest: "sha256:1234567890abcdef".to_string(),
        },
        HashMap::new(),
    );
    attestation.add_signature("trusted_developer".to_string(), "signature123".to_string());

    // Simulate policy check (this would normally be done in a service layer)
    let passes_policy = check_policy(&policy, &attestation);
    assert!(passes_policy, "Attestation should pass the policy");
}

#[test]
fn test_vulnerability_detection() {
    // Create a release
    let unmanaged = Unmanaged {
        id: Uuid::new_v4(),
        name: "Unmanaged Component".to_string(),
        repository_url: None,
        package_url: Some("https://example.com/packages/unmanaged-1.0.0.tar.gz".to_string()),
        metadata: HashMap::new(),
    };
    let component = SDLCComponent::Unmanaged(unmanaged);
    
    let release = SDLCRelease::<phase::Runtime, state::Deployed>::new(
        component,
        "1.0.0".to_string(),
        "integrator1".to_string()
    );

    // Create a policy
    let mut policy = Policy::new("Runtime Policy".to_string(), vec!["Runtime".to_string()]);
    policy.add_rule(PolicyRule::VulnerabilityThreshold(VulnerabilityLevel::High, 0));

    // Initially, the release passes the policy
    assert!(check_runtime_policy(&policy, &release), "Release should initially pass policy");

    // Simulate detection of a new vulnerability
    let vulnerability = Vulnerability {
        id: "CVE-2023-12345".to_string(),
        severity: VulnerabilityLevel::High,
        description: "Critical vulnerability detected".to_string(),
        discovered_at: Utc::now(),
    };

    // Update the release with the new vulnerability
    let updated_release = update_release_vulnerabilities(release, vec![vulnerability]);

    // Check policy again
    assert!(!check_runtime_policy(&policy, &updated_release), "Release should fail policy after vulnerability detection");
}

// Helper functions (these would typically be in a separate module)

fn check_policy(policy: &Policy, attestation: &Attestation) -> bool {
    policy.rules.iter().all(|rule| {
        match rule {
            PolicyRule::ApprovedIdentities(approved) => {
                attestation.signatures.iter().any(|sig| approved.contains(&sig.signer))
            },
            PolicyRule::MaxAge(max_age) => {
                let age = Utc::now() - attestation.timestamp;
                age <= ChronoDuration::from_std(*max_age).unwrap_or_else(|_| ChronoDuration::zero())
            },
            // Add checks for other rule types as needed
            _ => true, // Assume other rules pass for now
        }
    })
}

fn check_runtime_policy(policy: &Policy, release: &SDLCRelease<phase::Runtime, state::Deployed>) -> bool {
    policy.rules.iter().all(|rule| {
        match rule {
            PolicyRule::VulnerabilityThreshold(level, max_count) => {
                let count = release.state_info.vulnerabilities.iter()
                    .filter(|v| v.severity >= *level)
                    .count();
                count <= *max_count as usize
            },
            // Add checks for other rule types as needed
            _ => true, // Assume other rules pass for now
        }
    })
}

fn update_release_vulnerabilities(mut release: SDLCRelease<phase::Runtime, state::Deployed>, vulnerabilities: Vec<Vulnerability>) -> SDLCRelease<phase::Runtime, state::Deployed> {
    release.state_info.vulnerabilities.extend(vulnerabilities);
    release
}