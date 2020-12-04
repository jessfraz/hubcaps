//! Actions interface
use crate::workflows::Workflows;
use crate::Github;

pub struct Actions {
    github: Github,
    owner: String,
    repo: String,
}

impl Actions {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Actions {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// Return a reference to workflows operations
    pub fn workflows(&self) -> Workflows {
        Workflows::new(self.github.clone(), self.owner.clone(), self.repo.clone())
    }
}
