#[async_trait]
pub trait NamespaceService {
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