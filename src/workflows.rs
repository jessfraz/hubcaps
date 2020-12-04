//! Workflows interface
use std::collections::HashMap;

use serde::Serialize;

use crate::Future;
use crate::Github;

/// Provides access to worflows.
/// See the [github
/// docs](https://docs.github.com/en/free-pro-team@latest/rest/reference/actions#workflows)
/// for more information.
pub struct Workflows {
    github: Github,
    owner: String,
    repo: String,
}

impl Workflows {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Workflows {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, loc: &str) -> String {
        format!(
            "/repos/{}/{}/actions/workflows{}",
            self.owner, self.repo, loc
        )
    }

    /// Create a workflow dispatch event.
    ///
    /// See the [github docs](https://docs.github.com/en/free-pro-team@latest/rest/reference/actions#create-a-workflow-dispatch-event)
    /// for more information.
    /// `id`: The ID of the workflow. You can also pass the workflow file name as a string.
    pub fn dispatch(&self, id: &str, options: &WorkflowDispatchOptions) -> Future<()> {
        let uri = self.path(&format!("/{}/dispatches", id));
        self.github.post(&uri, json!(options))
    }
}

// representations

#[derive(Debug, Default, Serialize)]
pub struct WorkflowDispatchOptions {
    #[serde(rename = "ref")]
    pub refv: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub inputs: HashMap<String, String>,
}

impl WorkflowDispatchOptions {
    pub fn builder() -> WorkflowDispatchOptionsBuilder {
        WorkflowDispatchOptionsBuilder::default()
    }
}

#[derive(Default)]
pub struct WorkflowDispatchOptionsBuilder(WorkflowDispatchOptions);

impl WorkflowDispatchOptionsBuilder {
    /// Required. The git reference for the workflow. The reference can be a branch or tag name.
    pub fn reference<T>(&mut self, reference: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.refv = reference.into();
        self
    }

    /// Input keys and values configured in the workflow file. The maximum
    /// number of properties is 10. Any default properties configured in the
    /// workflow file will be used when inputs are omitted.
    pub fn inputs(&mut self, inputs: HashMap<String, String>) -> &mut Self {
        self.0.inputs = inputs;
        self
    }

    pub fn build(&self) -> WorkflowDispatchOptions {
        WorkflowDispatchOptions {
            inputs: self.0.inputs.clone(),
            refv: self.0.refv.clone(),
        }
    }
}
