use async_trait::async_trait;
use schemars::JsonSchema;
use sdlccp_api_macro::RegisterSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait NamespaceManager {
    /// Asynchronously creates a new namespace at the specified path.
    async fn create_namespace(&mut self, path: &str) -> Result<(), NamespaceError>;

    /// Asynchronously lists all child namespaces under the specified path.
    async fn list_namespaces(&self, path: &str) -> Result<Vec<String>, NamespaceError>;

    /// Asynchronously deletes the namespace at the specified path.
    async fn delete_namespace(&mut self, path: &str) -> Result<(), NamespaceError>;

    /// Asynchronously searches for namespaces matching the query.
    async fn search_namespaces(&self, query: &str) -> Result<Vec<String>, NamespaceError>;

    /// Asynchronously retrieves the namespace hierarchy starting from the specified path.
    async fn drill_down(&self, path: &str) -> Result<Arc<NamespaceNode>, NamespaceError>;
}

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize, JsonSchema, RegisterSchema)]
pub struct NamespaceNode {
    name: String,
    children: HashMap<String, Arc<NamespaceNode>>,
}

#[derive(Debug, Clone, ToSchema, JsonSchema, RegisterSchema)]
pub enum NamespaceError {
    NotFound,
    AlreadyExists,
    InvalidPath,
    PermissionDenied,
}

pub struct InMemoryNamespaceManager {
    root: Arc<RwLock<Arc<NamespaceNode>>>,
}

impl Default for InMemoryNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryNamespaceManager {
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(Arc::new(NamespaceNode {
                name: "".to_string(),
                children: HashMap::new(),
            }))),
        }
    }

    fn parse_path(path: &str) -> Vec<String> {
        path.trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

#[async_trait]
impl NamespaceManager for InMemoryNamespaceManager {
    async fn create_namespace(&mut self, path: &str) -> Result<(), NamespaceError> {
        let parts = Self::parse_path(path);
        if parts.is_empty() {
            return Err(NamespaceError::InvalidPath);
        }
    
        let mut root_guard = self.root.write().await;
        let mut current_node = Arc::make_mut(&mut *root_guard);
    
        for part in parts {
            current_node = Arc::make_mut(
                current_node
                    .children
                    .entry(part.clone())
                    .or_insert_with(|| Arc::new(NamespaceNode {
                        name: part.clone(),
                        children: HashMap::new(),
                    })),
            );
        }
    
        Ok(())
    }

    async fn list_namespaces(&self, path: &str) -> Result<Vec<String>, NamespaceError> {
        let parts = Self::parse_path(path);
        let hierarchy = self.root.read().await;
        let mut current_node = hierarchy.clone();

        for part in parts {
            let node = current_node
                .children
                .get(&part)
                .ok_or(NamespaceError::NotFound)?
                .clone();
            current_node = node;
        }

        let namespaces = current_node
            .children
            .keys()
            .cloned()
            .collect::<Vec<String>>();

        Ok(namespaces)
    }

    async fn search_namespaces(&self, query: &str) -> Result<Vec<String>, NamespaceError> {
        let hierarchy = self.root.read().await;
        let mut results = Vec::new();
        self.search_recursive(&hierarchy, "", query, &mut results);
        Ok(results)
    }

    async fn delete_namespace(&mut self, path: &str) -> Result<(), NamespaceError> {
        let parts = Self::parse_path(path);
        if parts.is_empty() {
            return Err(NamespaceError::InvalidPath);
        }

        let mut hierarchy = self.root.write().await;
        let mut current_node = Arc::make_mut(&mut hierarchy);

        for part in &parts[..parts.len() - 1] {
            current_node = Arc::make_mut(
                current_node
                    .children
                    .get_mut(part)
                    .ok_or(NamespaceError::NotFound)?,
            );
        }

        let removed = current_node
            .children
            .remove(&parts.last().unwrap().to_string());

        match removed {
            Some(_) => Ok(()),
            None => Err(NamespaceError::NotFound),
        }
    }

    async fn drill_down(&self, path: &str) -> Result<Arc<NamespaceNode>, NamespaceError> {
        let parts = Self::parse_path(path);
        let hierarchy = self.root.read().await;
        let mut current_node = hierarchy.clone();

        for part in parts {
            let node = current_node
                .children
                .get(&part)
                .ok_or(NamespaceError::NotFound)?
                .clone();
            current_node = node;
        }

        Ok(current_node)
    }
}

impl InMemoryNamespaceManager {
    fn search_recursive(
        &self,
        node: &Arc<NamespaceNode>,
        path: &str,
        query: &str,
        results: &mut Vec<String>,
    ) {
        let current_path = if path.is_empty() {
            node.name.clone()
        } else if node.name.is_empty() {
            path.to_string()
        } else {
            format!("{}/{}", path, node.name)
        };

        if node.name.contains(query) && !node.name.is_empty() {
            results.push(current_path.clone());
        }

        for child in node.children.values() {
            self.search_recursive(child, &current_path, query, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_and_list_namespaces() {
        let mut manager = InMemoryNamespaceManager::new();
        manager.create_namespace("a/b/c").await.unwrap();
        manager.create_namespace("a/b/d").await.unwrap();

        let namespaces = manager.list_namespaces("a/b").await.unwrap();
        assert_eq!(namespaces.len(), 2);
        assert!(namespaces.contains(&"c".to_string()));
        assert!(namespaces.contains(&"d".to_string()));
    }

    #[tokio::test]
    async fn test_delete_namespace() {
        let mut manager = InMemoryNamespaceManager::new();
        manager.create_namespace("x/y/z").await.unwrap();
        manager.delete_namespace("x/y").await.unwrap();

        let result = manager.list_namespaces("x").await;
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_search_namespaces() {
        let mut manager = InMemoryNamespaceManager::new();
        manager.create_namespace("search/test1").await.unwrap();
        manager.create_namespace("search/test2").await.unwrap();
        manager.create_namespace("other/test3").await.unwrap();

        let results = manager.search_namespaces("test").await.unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.contains(&"search/test1".to_string()));
        assert!(results.contains(&"search/test2".to_string()));
        assert!(results.contains(&"other/test3".to_string()));
    }
}
