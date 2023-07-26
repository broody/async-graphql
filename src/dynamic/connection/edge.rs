#[derive(Debug)]
pub struct Edge {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) inaccessible: bool,
    pub(crate) tags: Vec<String>,
}

impl Edge {
    /// Create a GraphQL connection type
    #[inline]
    pub fn new() -> Self {
        Self {
            name: "Edge".into(),
            description: None,
            inaccessible: true,
            tags: Vec::new(),
        }
    }

    impl_set_description!();
    impl_set_inaccessible!();
    impl_set_tags!();

    /// Returns the type name
    pub fn type_name(&self) -> &str {
        &self.name
    }
}
