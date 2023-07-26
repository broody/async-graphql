#[derive(Debug)]
pub struct Connection {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) inaccessible: bool,
    pub(crate) tags: Vec<String>,
}

impl Connection {
    /// Create a GraphQL connection type
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
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
        &format!("{}Connection", self.name)
    }
}
