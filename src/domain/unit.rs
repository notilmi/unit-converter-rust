#[derive(PartialEq, Debug, Clone)]
pub struct Unit {
    pub name: String,
}

impl Unit {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}
