use std::time::Duration;
use std::collections::HashMap;

use schemars::JsonSchema;
use sdlc_cp_api_macro::RegisterSchema;

#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub rules: Vec<PolicyRule>,
    pub parent_policies: Vec<String>, // Todo: This is currently IDs of parent policies, should this be a Vec<Uuid>, should this be some other way of referencing?
    pub applies_to: Vec<String>, // Todo: This currently is Phase names this policy applies to. Should this be a Vec<Phase>? Should this be some other way of referencing?
}

#[derive(Debug, Clone, JsonSchema)]
pub enum PolicyRule {
    MaxAge(Duration),
    ApprovedIdentities(Vec<String>),
    RequiredClaims(HashMap<String, String>),
    VulnerabilityThreshold(VulnerabilityLevel, u32),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, JsonSchema)]
pub enum VulnerabilityLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Policy {
    pub fn new(name: String, applies_to: Vec<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            rules: Vec::new(),
            parent_policies: Vec::new(),
            applies_to,
        }
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }
}