use schemars::JsonSchema;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::model::attestation::Attestation;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait AttestationService: Send + Sync {
    async fn store_attestation(&self, attestation: Attestation) -> Result<(), AttestationError>;
    async fn get_attestation(&self, id: &Uuid) -> Result<Option<Attestation>, AttestationError>;
    async fn get_attestations_for_release(&self, release_id: &Uuid) -> Result<Vec<Attestation>, AttestationError>;
    async fn verify_attestation(&self, attestation: &Attestation) -> Result<bool, AttestationError>;
}

#[derive(Debug, thiserror::Error, JsonSchema, ToSchema)]
pub enum AttestationError {
    #[error("Failed to store attestation: {0}")]
    StorageError(String),
    #[error("Failed to retrieve attestation: {0}")]
    RetrievalError(String),
    #[error("Failed to verify attestation: {0}")]
    VerificationError(String),
}

// Example in-memory implementation for testing
pub struct InMemoryAttestationService {
    attestations: Arc<RwLock<HashMap<Uuid, Attestation>>>,
}

impl InMemoryAttestationService {
    pub fn new() -> Self {
        Self {
            attestations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl AttestationService for InMemoryAttestationService {
    async fn store_attestation(&self, attestation: Attestation) -> Result<(), AttestationError> {
        let mut attestations = self.attestations.write().await;
        attestations.insert(attestation.id, attestation);
        Ok(())
    }

    async fn get_attestation(&self, id: &Uuid) -> Result<Option<Attestation>, AttestationError> {
        let attestations = self.attestations.read().await;
        Ok(attestations.get(id).cloned())
    }

    async fn get_attestations_for_release(&self, release_id: &Uuid) -> Result<Vec<Attestation>, AttestationError> {
        let attestations = self.attestations.read().await;
        Ok(attestations.values()
            .filter(|att| att.subject.name == release_id.to_string())
            .cloned()
            .collect())
    }

    async fn verify_attestation(&self, _attestation: &Attestation) -> Result<bool, AttestationError> {
        // TODO: Implement verification logic
        Ok(true)
    }
}
