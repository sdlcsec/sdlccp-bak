use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sdlc_cp_api_macro::RegisterSchema;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone, JsonSchema, RegisterSchema)]
pub struct Attestation {
    pub id: Uuid,
    pub subject: Subject,
    pub timestamp: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub signatures: Vec<Signature>,
    pub claims: HashMap<String, serde_json::Value>,
    pub parent_attestations: Vec<String>, // TODO: This is currently IDs of parent attestations. Should this be a Vec<Uuid>, should this be some other way of referencing?
}

#[derive(Clone, JsonSchema)]
pub struct Subject {
    pub type_: SubjectType,
    pub name: String,
    pub digest: String,
}

#[derive(Clone, JsonSchema)]
pub enum SubjectType {
    Commit,
    Artifact,
    Deployment,
}

#[derive(Clone, JsonSchema)]
pub struct Signature {
    pub signer: String,
    pub signature: String,
}

impl Attestation {
    pub fn new(subject: Subject, claims: HashMap<String, serde_json::Value>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            subject,
            timestamp: Utc::now(),
            expiration: None,
            signatures: Vec::new(),
            claims,
            parent_attestations: Vec::new(),
        }
    }

    pub fn add_signature(&mut self, signer: String, signature: String) {
        self.signatures.push(Signature { signer, signature });
    }
}