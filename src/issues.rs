//! Issues interface
use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

use crate::comments::Comments;
use crate::labels::Label;
use crate::users::{deserialize_null_user, User};
use crate::utils::{percent_encode, PATH_SEGMENT};
use crate::{Future, Github, SortDirection, Stream};

/// enum representation of github pull and issue state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    /// Only open issues
    Open,
    /// Only closed issues
    Closed,
    /// All issues, open or closed
    All,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::Open => "open",
            State::Closed => "closed",
            State::All => "all",
        }
        .fmt(f)
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}

/// Sort options available for github issues
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sort {
    /// sort by creation time of issue
    Created,
    /// sort by the last time issue was updated
    Updated,
    /// sort by number of comments
    Comments,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Sort::Created => "created",
            Sort::Updated => "updated",
            Sort::Comments => "comments",
        }
        .fmt(f)
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/// Provides access to assignee operations available for an individual issue
pub struct IssueAssignees {
    github: Github,
    owner: String,
    repo: String,
    number: u64,
}

impl IssueAssignees {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueAssignees {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}/assignees{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// add a set of assignees
    pub fn add(&self, assignees: Vec<&str>) -> Future<Issue> {
        self.github
            .post(&self.path(""), json_lit!({ "assignees": assignees }))
    }
}

/// Provides access to label operations available for an individual issue
pub struct IssueLabels {
    github: Github,
    owner: String,
    repo: String,
    number: u64,
}

impl IssueLabels {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueLabels {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}/labels{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// add a set of labels to this issue ref
    #[allow(clippy::needless_pass_by_value)] // shipped public API
    pub fn add(&self, labels: Vec<&str>) -> Future<Vec<Label>> {
        self.github.post(&self.path(""), json!(labels))
    }

    /// remove a label from this issue
    pub fn remove(&self, label: &str) -> Future<()> {
        let label = percent_encode(label.as_ref(), PATH_SEGMENT);
        self.github.delete(&self.path(&format!("/{}", label)))
    }

    /// replace all labels associated with this issue with a new set.
    /// providing an empty set of labels is the same as clearing the
    /// current labels
    #[allow(clippy::needless_pass_by_value)] // shipped public API
    pub fn set(&self, labels: Vec<&str>) -> Future<Vec<Label>> {
        self.github.put(&self.path(""), json!(labels))
    }

    /// remove all labels from an issue
    pub fn clear(&self) -> Future<()> {
        self.github.delete(&self.path(""))
    }
}

/// Provides access to operations available for a single issue
/// Typically accessed from `github.repo(.., ..).issues().get(number)`
pub struct IssueRef {
    github: Github,
    owner: String,
    repo: String,
    number: u64,
}

impl IssueRef {
    #[doc(hidden)]
    pub fn new<O, R>(github: Github, owner: O, repo: R, number: u64) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        IssueRef {
            github,
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    /// Request an issue's information
    pub fn get(&self) -> Future<Issue> {
        self.github.get(&self.path(""))
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}{}",
            self.owner, self.repo, self.number, more
        )
    }

    /// Return a reference to labels operations available for this issue
    pub fn labels(&self) -> IssueLabels {
        IssueLabels::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    /// Return a reference to assignee operations available for this issue
    pub fn assignees(&self) -> IssueAssignees {
        IssueAssignees::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            self.number,
        )
    }

    /// short hand for editing state = open
    pub fn open(&self) -> Future<Issue> {
        let mut o: IssueOptions = Default::default();
        o.state = Some("open".to_string());
        self.edit(&o)
    }

    /// shorthand for editing state = closed
    pub fn close(&self) -> Future<Issue> {
        let mut o: IssueOptions = Default::default();
        o.state = Some("closed".to_string());
        self.edit(&o)
    }

    /// Edit the issues options
    pub fn edit(&self, is: &IssueOptions) -> Future<Issue> {
        self.github.patch(&self.path(""), json!(is))
    }

    /// Return a reference to comment operations available for this issue
    pub fn comments(&self) -> Comments {
        Comments::new(
            self.github.clone(),
            self.owner.clone(),
            self.repo.clone(),
            self.number,
        )
    }
}

/// Provides access to operations available for a repository issues
/// Typically accessed via `github.repo(..., ...).issues()`
pub struct Issues {
    github: Github,
    owner: String,
    repo: String,
}

impl Issues {
    /// create a new instance of a github repo issue ref
    pub fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        Issues {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/issues{}", self.owner, self.repo, more)
    }

    pub fn get(&self, number: u64) -> IssueRef {
        IssueRef::new(
            self.github.clone(),
            self.owner.as_str(),
            self.repo.as_str(),
            number,
        )
    }

    pub fn create(&self, is: &IssueOptions) -> Future<Issue> {
        self.github.post(&self.path(""), json!(is))
    }

    /// Return the first page of issues for this repisotiry
    /// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
    /// for more information
    pub fn list(&self, options: &IssueListOptions) -> Future<Vec<Issue>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get(&uri.join("?"))
    }

    /// Return a stream of all issues for this repository
    ///
    /// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
    /// for more information
    ///
    /// Note: You'll typically want to use a `IssueListOptions` with a `per_page`
    /// of 100 for maximum api credential rate limit efficency
    pub fn iter(&self, options: &IssueListOptions) -> Stream<Issue> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.github.get_stream(&uri.join("?"))
    }
}

// representations

/// Options used to filter repository issue listings
///
/// See the [github docs](https://developer.github.com/v3/issues/#list-issues-for-a-repository)
/// for more information
///
/// By default this returns up to `30` items. You can
/// request up to `100` using the [per_page](https://developer.github.com/v3/#pagination)
/// parameter
#[derive(Default)]
pub struct IssueListOptions {
    params: HashMap<&'static str, String>,
}

impl IssueListOptions {
    pub fn builder() -> IssueListOptionsBuilder {
        IssueListOptionsBuilder::default()
    }

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

/// a mutable issue list builder
#[derive(Default)]
pub struct IssueListOptionsBuilder(IssueListOptions);

impl IssueListOptionsBuilder {
    pub fn state(&mut self, state: State) -> &mut Self {
        self.0.params.insert("state", state.to_string());
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Self {
        self.0.params.insert("sort", sort.to_string());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.direction(SortDirection::Asc)
    }

    pub fn desc(&mut self) -> &mut Self {
        self.direction(SortDirection::Desc)
    }

    pub fn direction(&mut self, direction: SortDirection) -> &mut Self {
        self.0.params.insert("direction", direction.to_string());
        self
    }

    pub fn assignee<A>(&mut self, assignee: A) -> &mut Self
    where
        A: Into<String>,
    {
        self.0.params.insert("assignee", assignee.into());
        self
    }

    pub fn creator<C>(&mut self, creator: C) -> &mut Self
    where
        C: Into<String>,
    {
        self.0.params.insert("creator", creator.into());
        self
    }

    pub fn mentioned<M>(&mut self, mentioned: M) -> &mut Self
    where
        M: Into<String>,
    {
        self.0.params.insert("mentioned", mentioned.into());
        self
    }

    pub fn labels<L>(&mut self, labels: Vec<L>) -> &mut Self
    where
        L: Into<String>,
    {
        self.0.params.insert(
            "labels",
            labels
                .into_iter()
                .map(|l| l.into())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.0.params.insert("since", since.into());
        self
    }

    pub fn per_page(&mut self, n: u32) -> &mut Self {
        self.0.params.insert("per_page", n.to_string());
        self
    }

    pub fn build(&self) -> IssueListOptions {
        IssueListOptions {
            params: self.0.params.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct IssueOptions {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl IssueOptions {
    pub fn new<T, B, A, L, S>(
        title: T,
        body: Option<B>,
        assignee: Option<A>,
        milestone: Option<u64>,
        labels: Vec<L>,
        state: Option<S>,
    ) -> IssueOptions
    where
        T: Into<String>,
        B: Into<String>,
        A: Into<String>,
        L: Into<String>,
        S: Into<String>,
    {
        IssueOptions {
            title: title.into(),
            body: body.map(|b| b.into()),
            assignee: assignee.map(|a| a.into()),
            milestone,
            labels: labels
                .into_iter()
                .map(|l| l.into())
                .collect::<Vec<String>>(),
            state: state.map(|s| s.into()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub labels: Vec<Label>,
    pub assignee: Option<User>,
    pub locked: bool,
    pub comments: u64,
    pub pull_request: Option<PullRef>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assignees: Vec<User>,
    #[serde(default, deserialize_with = "deserialize_null_user::deserialize")]
    pub closed_by: User,
}

/// A reference to a pull request.
#[derive(Debug, Clone, Deserialize)]
pub struct PullRef {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state() {
        let default: State = Default::default();
        assert_eq!(default, State::Open)
    }

    #[test]
    fn issue_list_reqs() {
        fn test_serialize(tests: Vec<(IssueListOptions, Option<String>)>) {
            for test in tests {
                let (k, v) = test;
                assert_eq!(k.serialize(), v);
            }
        }
        let tests = vec![
            (IssueListOptions::builder().build(), None),
            (
                IssueListOptions::builder().state(State::Closed).build(),
                Some("state=closed".to_owned()),
            ),
            (
                IssueListOptions::builder()
                    .labels(vec!["foo", "bar"])
                    .build(),
                Some("labels=foo%2Cbar".to_owned()),
            ),
        ];
        test_serialize(tests)
    }

    #[test]
    fn sort_default() {
        let default: Sort = Default::default();
        assert_eq!(default, Sort::Created)
    }

    #[test]
    fn sort_display() {
        for (k, v) in &[
            (Sort::Created, "created"),
            (Sort::Updated, "updated"),
            (Sort::Comments, "comments"),
        ] {
            assert_eq!(k.to_string(), *v)
        }
    }
}
