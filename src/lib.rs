pub mod model;
pub mod services;

pub use sdlccp_api_macro::RegisterSchema;

pub struct SchemaGenerator {
    pub type_name: &'static str,
    pub generator: fn() -> schemars::schema::RootSchema,
}

impl SchemaGenerator {
    pub const fn new(type_name: &'static str, generator: fn() -> schemars::schema::RootSchema) -> Self {
        Self { type_name, generator }
    }
}

inventory::collect!(SchemaGenerator);

#[cfg(test)]
mod tests {
    mod model_tests;
}