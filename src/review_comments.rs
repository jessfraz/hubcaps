//! Review comments interface
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

use crate::users::User;
use crate::{Future, Github, Stream};

/// A structure for interfacing with a review comments
pub struct ReviewComments {
    github: Github,
    owner: String,
    repo: String,
    number: u64,
}

impl ReviewComments {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        ReviewComments {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// list review comments
    pub fn list(&self, options: &ReviewCommentListOptions) -> Future<Vec<ReviewComment>> {
        let mut uri = vec![self.path()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// provides a stream over all pages of the review comments
    pub fn iter(&self, options: &ReviewCommentListOptions) -> Stream<ReviewComment> {
        let mut uri = vec![self.path()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get_stream(&uri.join("?"))
    }

    /// Create new review comment
    pub fn create(&self, review_comment: &ReviewCommentOptions) -> Future<ReviewComment> {
        self.github.post(&self.path(), json!(review_comment))
    }

    fn path(&self) -> String {
        format!(
            "/repos/{}/{}/pulls/{}/comments",
            self.owner, self.repo, self.number
        )
    }
}

// representations (todo: replace with derive_builder)

#[derive(Default, Serialize)]
pub struct ReviewCommentOptions {
    pub body: String,
    pub commit_id: String,
    pub path: String,
    pub position: usize,
}

#[derive(Debug, Deserialize)]
pub struct ReviewComment {
    pub id: u64,
    pub url: String,
    pub diff_hunk: String,
    pub path: String,
    #[serde(default)]
    pub position: u64,
    #[serde(default)]
    pub original_position: u64,
    pub commit_id: String,
    pub original_commit_id: String,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub html_url: String,
    pub pull_request_url: String,
}

#[derive(Default)]
pub struct ReviewCommentListOptions {
    params: HashMap<&'static str, String>,
}

impl ReviewCommentListOptions {
    pub fn builder() -> ReviewCommentListOptionsBuilder {
        ReviewCommentListOptionsBuilder::default()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
    }
}

#[derive(Default)]
pub struct ReviewCommentListOptionsBuilder(ReviewCommentListOptions);

impl ReviewCommentListOptionsBuilder {
    pub fn per_page(&mut self, n: usize) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.0.params.insert("since", since.into());
        self
    }

    pub fn build(&self) -> ReviewCommentListOptions {
        ReviewCommentListOptions {
            params: self.0.params.clone(),
        }
    }
}
