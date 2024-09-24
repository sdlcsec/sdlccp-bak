use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use axum::{extract::State, http::request, response::IntoResponse, routing, Json, Router};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use uuid::{Timestamp, Uuid};

use crate::model::{
    attestation::{Signature, Subject, SubjectType},
    phase::{
        BuildDetails, DeployDetails, DevelopmentDetails, PackageDetails, PhaseDetails,
        RuntimeDetails, SourceDetails,
    },
    policy::{Policy, PolicyRule, Vulnerability, VulnerabilityLevel},
    sdlc_component::{Project, SDLCComponent, Unmanaged},
    Attestation, ReleaseState, SDLCPhase, SDLCRelease,
};

use super::{attestation::AttestationError, namespace::{InMemoryNamespaceManager, NamespaceManager, NamespaceNode}, policy_repository::PolicyRepositoryError};

#[derive(OpenApi)]
#[openapi(
    paths(
        create_policy,
        get_policy,
        get_policies_for_component,
        create_attestation,
        get_attestation,
        get_attestations_for_release,
        apply_policy_to_release,
        create_namespace
    ),
    components(schemas(
        SDLCComponent,
        Project,
        Unmanaged,
        SDLCRelease,
        ReleaseState,
        VulnerabilityLevel,
        Attestation,
        AttestationError,
        Policy,
        PolicyRepositoryError,
        Signature,
        Subject,
        SubjectType,
        PolicyRule,
        ControlPlaneError,
        SDLCPhase,
        PhaseDetails,
        RuntimeDetails,
        SourceDetails,
        DevelopmentDetails,
        DeployDetails,
        BuildDetails,
        PackageDetails,
        Vulnerability,
        AttestationCreateRequest,
        NamespaceCreateRequest,
        NamespaceCreateResponse,
        NamespaceCreateError,
        NamespaceDeleteResponse,
        NamespaceDeleteError,
        NamespaceListResponse,
        NamespaceListError,
        NamespaceGetResponse,
        NamespaceGetError,
        NamespaceSearchError,
    )),
    tags(
        (name = "policies", description = "Policy management endpoints"),
        (name = "attestations", description = "Attestation management endpoints"),
        (name = "releases", description = "Release management endpoints")
    )
)]
pub struct ControlPlaneAPIDoc;

#[async_trait]
pub trait ControlPlane {
    async fn apply_policy_to_release(
        &self,
        release_id: &Uuid,
        component_id: &Uuid,
    ) -> Result<bool, ControlPlaneError>;
    async fn store_policy(&self, policy: Policy) -> Result<(), ControlPlaneError>;
    async fn get_policy(&self, id: &Uuid) -> Result<Option<Policy>, ControlPlaneError>;
    async fn store_attestation(&self, attestation: Attestation) -> Result<(), ControlPlaneError>;
    async fn get_attestation(&self, id: &Uuid) -> Result<Option<Attestation>, ControlPlaneError>;
}

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum ControlPlaneError {
    #[error("Policy repository error: {0}")]
    PolicyRepositoryError(String),
    #[error("Attestation storage error: {0}")]
    AttestationStorageError(String),
    #[error("No policy found for component")]
    NoPolicyFound,
}

#[utoipa::path(
    post,
    path = "/policies",
    request_body = Policy,
    responses(
        (status = 201, description = "Policy created successfully", body = Policy),
        (status = 400, description = "Invalid policy data", body = PolicyRepositoryError)
    ),
    tag = "policies"
)]
pub async fn create_policy(_policy: Policy) -> Result<Policy, PolicyRepositoryError> {
    // Implementation here
    unimplemented!()
}

#[utoipa::path(
    get,
    path = "/policies/{id}",
    responses(
        (status = 200, description = "Policy found", body = Policy),
        (status = 404, description = "Policy not found", body = PolicyRepositoryError)
    ),
    params(
        ("id" = Uuid, Path, description = "Policy ID")
    ),
    tag = "policies"
)]
pub async fn get_policy(_id: Uuid) -> Result<Policy, PolicyRepositoryError> {
    // Implementation here
    unimplemented!()
}

#[utoipa::path(
    get,
    path = "/components/{id}/policies",
    responses(
        (status = 200, description = "Policies found", body = Vec<Policy>),
        (status = 404, description = "Component not found", body = PolicyRepositoryError)
    ),
    params(
        ("id" = Uuid, Path, description = "Component ID")
    ),
    tag = "policies"
)]
pub async fn get_policies_for_component(_id: Uuid) -> Result<Vec<Policy>, PolicyRepositoryError> {
    // Implementation here
    unimplemented!()
}

#[derive(Clone, Deserialize, JsonSchema, ToSchema)]
pub struct AttestationCreateRequest {
    pub subject: Subject,
    pub expiration: Option<DateTime<Utc>>,
    pub signatures: Vec<Signature>,
    pub claims: HashMap<String, serde_json::Value>,
    pub parent_attestations: Vec<String>, // Use strings for client-provided UUIDs
}

#[utoipa::path(
    post,
    path = "/attestations",
    request_body = Attestation,
    responses(
        (status = 201, description = "Attestation created successfully", body = Attestation),
        (status = 400, description = "Invalid attestation data", body = AttestationError)
    ),
    tag = "attestations"
)]
pub async fn create_attestation(
    _attestation: AttestationCreateRequest,
) -> Result<Attestation, AttestationError> {
    // Implementation here
    Ok(Attestation {
        id: uuid::Uuid::new_v4(),
        subject: todo!(),
        timestamp: todo!(),
        expiration: todo!(),
        signatures: todo!(),
        claims: todo!(),
        parent_attestations: todo!(),
    })
}

#[utoipa::path(
    get,
    path = "/attestations/{id}",
    responses(
        (status = 200, description = "Attestation found", body = Attestation),
        (status = 404, description = "Attestation not found", body = AttestationError)
    ),
    params(
        ("id" = Uuid, Path, description = "Attestation ID")
    ),
    tag = "attestations"
)]
pub async fn get_attestation(_id: Uuid) -> Result<Attestation, AttestationError> {
    // Implementation here
    unimplemented!()
}

#[utoipa::path(
    get,
    path = "/releases/{id}/attestations",
    responses(
        (status = 200, description = "Attestations found", body = Vec<Attestation>),
        (status = 404, description = "Release not found", body = AttestationError)
    ),
    params(
        ("id" = Uuid, Path, description = "Release ID")
    ),
    tag = "releases"
)]
pub async fn get_attestations_for_release(_id: Uuid) -> Result<Vec<Attestation>, AttestationError> {
    // Implementation here
    unimplemented!()
}

pub type NamespaceStore = Mutex<InMemoryNamespaceManager>;

#[utoipa::path(
    post,
    path = "/releases/{release_id}/apply-policy/{policy_id}",
    responses(
        (status = 200, description = "Policy applied successfully", body = bool),
        (status = 400, description = "Invalid policy or release data", body = ControlPlaneError),
        (status = 404, description = "Release or policy not found", body = ControlPlaneError)
    ),
    params(
        ("release_id" = Uuid, Path, description = "Release ID"),
        ("policy_id" = Uuid, Path, description = "Policy ID")
    ),
    tag = "releases"
)]
pub async fn apply_policy_to_release(
    _release_id: Uuid,
    _policy_id: Uuid,
) -> Result<bool, ControlPlaneError> {
    // Implementation here
    unimplemented!()
}


#[utoipa::path(
    post,
    path = "/",
    request_body = NamespaceCreateRequest,
    responses(
        (status = 201, description = "Namespace created successfully", body = NamespaceCreateResponse),
        (status = 400, description = "Invalid namespace data", body = NamespaceCreateError)
    ),
    tag = ""
)]
pub async fn create_namespace(State(store): State<Arc<NamespaceStore>>,
Json(namespace_create_request): Json<NamespaceCreateRequest>,
) -> impl IntoResponse { //Result<Json<NamespaceCreateResponse>, Json<NamespaceCreateError>> {
    let namespace_create_result = store.lock().await.create_namespace(namespace_create_request.namespace.as_str()).await;
    match namespace_create_result {
        Ok(_) => Ok(Json(NamespaceCreateResponse)),
        Err(_) => Err(Json(NamespaceCreateError::InvalidPath)),
    }
}

#[derive(Clone, Deserialize, JsonSchema, ToSchema)]
pub struct NamespaceCreateRequest {
    pub namespace: String
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct NamespaceCreateResponse;

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
pub enum NamespaceCreateError {
    InvalidPath,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 201, description = "Namespace created successfully", body = NamespaceListResponse),
        (status = 400, description = "Invalid namespace data", body = NamespaceListError)
    ),
    tag = ""
)]
pub async fn list_namespaces(State(store): State<Arc<NamespaceStore>>) -> impl IntoResponse {
    let namespace_list_result = store.lock().await.list_namespaces("").await;
    match namespace_list_result {
        Ok(namespaces) => Ok(Json(NamespaceListResponse { namespaces })),
        Err(_) => Err(Json(NamespaceListError::InvalidPath)),
    }
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
struct NamespaceListResponse {
    namespaces: Vec<String>
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
enum NamespaceListError {
    InvalidPath,
}

pub async fn search_namespaces(State(store): State<Arc<NamespaceStore>>,
query: String) -> impl IntoResponse {
    let namespace_search_result = store.lock().await.search_namespaces(query.as_str()).await;
    match namespace_search_result {
        Ok(namespaces) => Ok(Json(NamespaceListResponse { namespaces })),
        Err(_) => Err(Json(NamespaceSearchError::InvalidPath)),
    }
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
enum NamespaceSearchError {
    InvalidPath,
}

pub async fn get_namespace(State(store): State<Arc<NamespaceStore>>,
request: request::Parts,
) -> impl IntoResponse {
    let namespace_path = request
        .uri
        .path()
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/");
    let namespace_drill_down_result = store.lock().await.drill_down(namespace_path.as_str()).await;
    match namespace_drill_down_result {
        Ok(namespace) => Ok(Json(NamespaceGetResponse { namespace })),
        Err(_) => Err(Json(NamespaceGetError::NamespaceNotFound)),
    }
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
struct NamespaceGetResponse {
    namespace: Arc<NamespaceNode>
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
enum NamespaceGetError {
    NamespaceNotFound
}

pub async fn delete_namespace(State(store): State<Arc<NamespaceStore>>,
request: request::Parts,
) -> impl IntoResponse {
    let namespace_path = request
        .uri
        .path()
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/");
    let namespace_delete_result = store.lock().await.delete_namespace(namespace_path.as_str()).await;
    match namespace_delete_result {
        Ok(_) => Ok(Json(NamespaceDeleteResponse)),
        Err(_) => Err(Json(NamespaceDeleteError::NamespaceNotFound)),
    }
}

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
struct NamespaceDeleteResponse;

#[derive(Clone, Deserialize, Serialize, JsonSchema, ToSchema)]
enum NamespaceDeleteError {
    NamespaceNotFound
}

pub fn namespace_router() -> Router {
    let store = Arc::new(NamespaceStore::default());
    Router::new()
        .route("/", routing::get(list_namespaces).post(create_namespace))
        .route("/search", routing::get(search_namespaces))
        .route("/*namespace_path", routing::get(get_namespace).delete(delete_namespace))
        .with_state(store)
}