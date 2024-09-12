use async_trait::async_trait;
use schemars::JsonSchema;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::Policy;

#[async_trait]
pub trait PolicyRepository: Send + Sync {
    async fn store_policy(&self, policy: Policy) -> Result<(), PolicyRepositoryError>;
    async fn get_policy(&self, id: &Uuid) -> Result<Option<Policy>, PolicyRepositoryError>;
    async fn get_policies_for_component(&self, component_id: &Uuid) -> Result<Vec<Policy>, PolicyRepositoryError>;
    async fn get_latest_policy_for_component(&self, component_id: &Uuid) -> Result<Option<Policy>, PolicyRepositoryError>;
}

#[derive(Debug, thiserror::Error, JsonSchema, ToSchema)]
pub enum PolicyRepositoryError {
    #[error("Failed to store policy: {0}")]
    StorageError(String),
    #[error("Failed to retrieve policy: {0}")]
    RetrievalError(String),
}

