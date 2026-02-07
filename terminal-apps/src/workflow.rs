use serde::{Deserialize, Serialize};
use thiserror::Error;
use warp_workflows_types::{Argument, Workflow};

/// Simple glob pattern matching
fn glob_match(text: &str, pattern: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    fn matches(text: &[char], pattern: &[char]) -> bool {
        match (text, pattern) {
            (_, []) => text.is_empty(),
            ([], ['*', rest @ ..]) => matches(&[], rest),
            ([], _) => false,
            (t, ['*', rest @ ..]) => matches(t, rest) || matches(&t[1..], pattern),
            ([t, rest_t @ ..], [p, rest_p @ ..]) if t == p || *p == '?' => {
                matches(rest_t, rest_p)
            }
            _ => false,
        }
    }

    matches(&text_chars, &pattern_chars)
}

/// Workflow execution context
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub variables: std::collections::HashMap<String, String>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        ExecutionContext {
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Conditional logic for workflow steps
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Condition {
    pub variable: String,
    pub operator: String, // "equals", "contains", "exists", "matches"
    pub value: Option<String>,
}

impl Condition {
    pub fn evaluate(&self, context: &ExecutionContext) -> bool {
        match self.operator.as_str() {
            "exists" => context.get_variable(&self.variable).is_some(),
            "equals" => {
                if let Some(val) = &self.value {
                    context.get_variable(&self.variable).map_or(false, |v| v == val)
                } else {
                    false
                }
            }
            "contains" => {
                if let Some(val) = &self.value {
                    context
                        .get_variable(&self.variable)
                        .map_or(false, |v| v.contains(val))
                } else {
                    false
                }
            }
            "matches" => {
                if let Some(pattern) = &self.value {
                    if let Some(v) = context.get_variable(&self.variable) {
                        // Simple glob pattern matching
                        return glob_match(v, pattern);
                    }
                }
                false
            }
            _ => false,
        }
    }
}

/// A workflow step that can be sequential or conditional
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowStep {
    pub name: String,
    pub workflow: String,
    pub condition: Option<Condition>,
    pub on_success: Option<Vec<String>>, // Next steps to execute
    pub on_failure: Option<Vec<String>>, // Steps to execute on failure
}

/// Extended workflow type with conditional logic
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExtendedWorkflow {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
}

impl ExtendedWorkflow {
    pub fn new(name: impl Into<String>) -> Self {
        ExtendedWorkflow {
            name: name.into(),
            description: None,
            steps: Vec::new(),
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }

    pub fn set_variable(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }
}

/// Workflow execution errors
#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Workflow not found: {0}")]
    NotFound(String),
    #[error("Step not found: {0}")]
    StepNotFound(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Condition evaluation error: {0}")]
    ConditionError(String),
}

pub type WorkflowResult<T> = Result<T, WorkflowError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_context() {
        let mut ctx = ExecutionContext::new();
        ctx.set_variable("status", "success");

        assert_eq!(ctx.get_variable("status"), Some(&"success".to_string()));
    }

    #[test]
    fn test_condition_exists() {
        let mut ctx = ExecutionContext::new();
        ctx.set_variable("file", "/tmp/test");

        let cond = Condition {
            variable: "file".to_string(),
            operator: "exists".to_string(),
            value: None,
        };

        assert!(cond.evaluate(&ctx));
    }

    #[test]
    fn test_condition_equals() {
        let mut ctx = ExecutionContext::new();
        ctx.set_variable("status", "success");

        let cond = Condition {
            variable: "status".to_string(),
            operator: "equals".to_string(),
            value: Some("success".to_string()),
        };

        assert!(cond.evaluate(&ctx));
    }

    #[test]
    fn test_extended_workflow() {
        let mut wf = ExtendedWorkflow::new("deploy");
        wf.set_variable("env", "production");

        assert_eq!(wf.variables.get("env"), Some(&"production".to_string()));
    }
}
