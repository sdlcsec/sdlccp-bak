use crate::model::*;
use attestation::{Attestation, Subject, SubjectType};
use chrono::Utc;
use phase::{PhaseDetails, RuntimeDetails};
use policy::{Policy, PolicyRule, Vulnerability, VulnerabilityLevel};
use sdlc_component::{Project, SDLCComponent, Unmanaged};
use std::collections::HashMap;
use uuid::Uuid;

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
    let mut release = SDLCRelease::new(
        component,
        "1.0.0".to_string(),
        "developer1".to_string()
    );

    assert_eq!(release.phase, SDLCPhase::Development);
    assert_eq!(release.state, ReleaseState::Draft);
    assert_eq!(release.created_by, "developer1");
    assert_eq!(release.component_name(), "Test Project");

    // Start development
    release.start_development("developer2".to_string(), vec!["feature x".to_string(), "feature y".to_string()]).unwrap();
    assert_eq!(release.phase, SDLCPhase::Development);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));
    if let ReleaseState::InProgress { started_by, .. } = &release.state {
        assert_eq!(started_by, "developer2");
    }

    // Complete development
    release.complete_development().unwrap();
    assert_eq!(release.phase, SDLCPhase::Source);
    assert!(matches!(release.state, ReleaseState::Draft));

    // Start source review
    release.start_source_review("reviewer1".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Source);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete source review
    release.complete_source_review("abcdef123456".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Build);
    assert!(matches!(release.state, ReleaseState::Draft));
    assert_eq!(release.commit_hash, Some("abcdef123456".to_string()));

    // Start build
    release.start_build("builder1".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Build);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete build
    release.complete_build("build123".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Package);
    assert!(matches!(release.state, ReleaseState::Draft));

    // Start packaging
    release.start_packaging("packager1".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Package);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete packaging
    release.complete_packaging("123abc456def".to_string(), "https://example.com/artifacts/project1-1.0.0.tar.gz".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Deploy);
    assert!(matches!(release.state, ReleaseState::Releasable { .. }));

    // Release
    release.release("Version 1.0.0 release notes".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Deploy);
    assert!(matches!(release.state, ReleaseState::Released { .. }));

    // Start deployment
    release.start_deployment("production".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Deploy);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete deployment
    release.complete_deployment().unwrap();
    assert_eq!(release.phase, SDLCPhase::Runtime);
    assert!(matches!(release.state, ReleaseState::Deployed { .. }));

    // Revoke
    release.revoke("Critical bug found".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Runtime);
    assert!(matches!(release.state, ReleaseState::Revoked { .. }));
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
    let mut release = SDLCRelease::new(
        component,
        "1.0.0".to_string(),
        "integrator1".to_string()
    );

    assert_eq!(release.phase, SDLCPhase::Development);
    assert_eq!(release.state, ReleaseState::Draft);
    assert_eq!(release.created_by, "integrator1");
    assert_eq!(release.component_name(), "Unmanaged Component");

    // For unmanaged components, we might skip some phases or handle them differently
    // Here's an example of a simplified lifecycle:

    // Start integration (using development phase)
    release.start_development("integrator2".to_string(), vec!["feature 1".to_string(), "feature 2".to_string()]).unwrap();
    assert_eq!(release.phase, SDLCPhase::Development);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete integration
    release.complete_development().unwrap();
    assert_eq!(release.phase, SDLCPhase::Source);
    assert!(matches!(release.state, ReleaseState::Draft));

    // Start source review (which might be just a verification process for unmanaged components)
    release.start_source_review("verifier1".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Source);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete source review
    release.complete_source_review("verify123".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Build);
    assert!(matches!(release.state, ReleaseState::Draft));

    // For unmanaged components, build and packaging might be skipped
    // Move directly to Deploy phase
    release.phase = SDLCPhase::Deploy;
    release.state = ReleaseState::Releasable { approved_by: "Auto-Approved".to_string(), approved_at: Utc::now() };

    // Release
    release.release("Unmanaged Component 1.0.0 integrated".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Deploy);
    assert!(matches!(release.state, ReleaseState::Released { .. }));

    // Start deployment
    release.start_deployment("production".to_string()).unwrap();
    assert_eq!(release.phase, SDLCPhase::Deploy);
    assert!(matches!(release.state, ReleaseState::InProgress { .. }));

    // Complete deployment
    release.complete_deployment().unwrap();
    assert_eq!(release.phase, SDLCPhase::Runtime);
    assert!(matches!(release.state, ReleaseState::Deployed { .. }));
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
    
    let mut release = SDLCRelease::new(
        component,
        "1.0.0".to_string(),
        "integrator1".to_string()
    );

    // Set the release to Runtime phase and Deployed state
    release.phase = SDLCPhase::Runtime;
    release.state = ReleaseState::Deployed { environment: "production".to_string(), deployment_time: Utc::now() };

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
                age <= chrono::Duration::from_std(*max_age).unwrap_or_else(|_| chrono::Duration::zero())
            },
            // Add checks for other rule types as needed
            _ => true, // Assume other rules pass for now
        }
    })
}

fn check_runtime_policy(policy: &Policy, release: &SDLCRelease) -> bool {
    if let ReleaseState::Deployed { .. } = release.state {
        policy.rules.iter().all(|rule| {
            match rule {
                PolicyRule::VulnerabilityThreshold(level, max_count) => {
                    if let Some(PhaseDetails { runtime_details: Some(runtime_details), .. }) = &release.phase_details {
                        let count = runtime_details.vulnerabilities.iter()
                            .filter(|v| v.severity >= *level)
                            .count();
                        count <= *max_count as usize
                    } else {
                        true // No runtime details available, assume it passes
                    }
                },
                // Add checks for other rule types as needed
                _ => true, // Assume other rules pass for now
            }
        })
    } else {
        false // Release is not in Deployed state
    }
}

fn update_release_vulnerabilities(mut release: SDLCRelease, vulnerabilities: Vec<Vulnerability>) -> SDLCRelease {
    if let Some(PhaseDetails { runtime_details: Some(runtime_details), .. }) = &mut release.phase_details {
        runtime_details.vulnerabilities.extend(vulnerabilities);
    } else {
        // If there are no runtime details, create them
        release.phase_details = Some(PhaseDetails {
            build_details: None, // or appropriate default value
            custom_details: HashMap::new(), // or appropriate default value
            deploy_details: None, // or appropriate default value
            runtime_details: Some(RuntimeDetails {
                runtime_id: Uuid::new_v4().to_string(),
                last_heartbeat: Utc::now(),
                vulnerabilities,
            }),
            development_details: None,
            source_details: None,
            package_details: None,
        });
    }
    release
}