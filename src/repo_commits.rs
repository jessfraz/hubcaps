//! Repo Commits interface
//! https://developer.github.com/v3/repos/commits/#get-a-single-commit
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::checks::{CheckSuiteListOptions, CheckSuiteResponse};
use crate::users::{deserialize_null_user, User};
use crate::{Future, Github, Stream};

/// A structure for interfacing with a repository commits
pub struct RepoCommits {
    github: Github,
    owner: String,
    repo: String,
}

impl RepoCommits {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        RepoCommits {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list repo commits
    /// !!! make optional parameters
    pub fn list(
        &self,
        path: &str,
        commit_ref: &str,
        since: Option<DateTime<Utc>>,
    ) -> Future<Vec<RepoCommit>> {
        let mut uri = format!("/repos/{}/{}/commits?&per_page=100", self.owner, self.repo);
        if !path.is_empty() {
            uri += &format!("&path={}", path);
        }
        if let Some(date) = since {
            uri += &format!("&since={}", date.to_rfc3339());
        }
        if !commit_ref.is_empty() {
            uri += &format!("&sha={}", commit_ref);
        }
        self.github.get::<Vec<RepoCommit>>(&uri)
    }

    /// provides a stream over all pages of pull commits
    /// !!! make optional parameters
    pub fn iter(&self) -> Stream<RepoCommit> {
        self.github
            .get_stream(&format!("/repos/{}/{}/commits", self.owner, self.repo))
    }

    /// get a repo commit
    pub fn get(&self, commit_ref: &str) -> Future<RepoCommit> {
        let uri = format!("/repos/{}/{}/commits/{}", self.owner, self.repo, commit_ref);
        self.github.get::<RepoCommit>(&uri)
    }

    /// provides a stream over all pages of check suites for a commit
    /// !!! make optional parameters
    pub fn list_check_suites(
        &self,
        ref_: &str,
        options: &CheckSuiteListOptions,
    ) -> Future<CheckSuiteResponse> {
        let mut uri = vec![format!(
            "/repos/{}/{}/commits/{}/check-suites",
            self.owner, self.repo, ref_
        )];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }
}

// representations

// !!! RepoCommit, CommitDetails, CommitRef, UserStamp are exact
//     dupes of pull_commits.rs' representations.

/// Representation of a repo commit
#[derive(Debug, Deserialize)]
pub struct RepoCommit {
    pub url: String,
    pub sha: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitDetails,
    #[serde(default, deserialize_with = "deserialize_null_user::deserialize")]
    pub author: User,
    #[serde(default, deserialize_with = "deserialize_null_user::deserialize")]
    pub committer: User,
    pub parents: Vec<CommitRef>,
    #[serde(default)]
    pub files: Vec<File>,
    #[serde(default)]
    pub stats: Stats,
}

/// Representation of a repo commit details
#[derive(Debug, Deserialize)]
pub struct CommitDetails {
    pub url: String,
    pub author: UserStamp,
    pub committer: Option<UserStamp>,
    pub message: String,
    pub tree: CommitRef,
    #[serde(default)]
    pub comment_count: u64,
}

/// Representation of a reference to a commit
#[derive(Debug, Deserialize)]
pub struct CommitRef {
    pub url: String,
    pub sha: String,
}

/// Representation of a git user
#[derive(Debug, Deserialize)]
pub struct UserStamp {
    pub name: String,
    pub email: String,
    pub date: DateTime<Utc>,
}

/// Representation of commit stats
#[derive(Debug, Default, Deserialize)]
pub struct Stats {
    pub additions: i64,
    pub deletions: i64,
    pub total: i64,
}

/// Representation of a file that changed in a commit
#[derive(Debug, Deserialize)]
pub struct File {
    pub filename: String,
    pub status: String,
}
