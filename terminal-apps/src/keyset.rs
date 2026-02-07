use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Represents a key binding configuration
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct KeyBinding {
    pub action: String,
    pub key: String,
    pub description: Option<String>,
}

/// Represents a complete keyset configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeySet {
    pub name: String,
    pub description: Option<String>,
    pub base: Option<String>, // Can inherit from another keyset
    #[serde(flatten)]
    pub bindings: HashMap<String, String>,
}

impl KeySet {
    pub fn new(name: impl Into<String>) -> Self {
        KeySet {
            name: name.into(),
            description: None,
            base: None,
            bindings: HashMap::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    pub fn add_binding(&mut self, action: impl Into<String>, key: impl Into<String>) {
        self.bindings.insert(action.into(), key.into());
    }

    pub fn get_binding(&self, action: &str) -> Option<&String> {
        self.bindings.get(action)
    }

    pub fn list_bindings(&self) -> Vec<(&String, &String)> {
        self.bindings.iter().collect()
    }

    /// Merge another keyset into this one
    pub fn merge(&mut self, other: KeySet) {
        for (action, key) in other.bindings {
            self.bindings.insert(action, key);
        }
    }
}

/// Keyset management errors
#[derive(Error, Debug)]
pub enum KeySetError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("KeySet not found: {0}")]
    NotFound(String),
    #[error("Invalid key binding: {0}")]
    InvalidBinding(String),
}

pub type KeySetResult<T> = Result<T, KeySetError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyset_creation() {
        let mut ks = KeySet::new("vim");
        ks.add_binding("editor:save", "ctrl-s");
        ks.add_binding("editor:undo", "ctrl-z");

        assert_eq!(ks.get_binding("editor:save"), Some(&"ctrl-s".to_string()));
        assert_eq!(ks.get_binding("editor:undo"), Some(&"ctrl-z".to_string()));
    }

    #[test]
    fn test_keyset_merge() {
        let mut ks1 = KeySet::new("base");
        ks1.add_binding("action1", "key1");

        let mut ks2 = KeySet::new("custom");
        ks2.add_binding("action2", "key2");

        ks1.merge(ks2);
        assert_eq!(ks1.bindings.len(), 2);
    }
}
