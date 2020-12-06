//! Checks interface
use std::collections::HashMap;

// see: https://developer.github.com/v3/checks/suites/
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

use self::super::{AuthenticationConstraint, Future, Github, MediaType};

pub struct CheckRuns {
    github: Github,
    owner: String,
    repo: String,
}

impl<'a> CheckRuns {
    #[doc(hidden)]
    pub(crate) fn new<O, R>(github: Github, owner: O, repo: R) -> Self
    where
        O: Into<String>,
        R: Into<String>,
    {
        CheckRuns {
            github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/check-runs{}", self.owner, self.repo, more)
    }

    pub fn create(&self, check_run_options: &CheckRunOptions) -> Future<CheckRun> {
        match serde_json::to_string(check_run_options) {
            Ok(data) => self.github.post_media::<CheckRun>(
                &self.path(""),
                data.into_bytes(),
                MediaType::Preview("antiope"),
                AuthenticationConstraint::Unconstrained,
            ),
            Err(e) => Box::pin(futures::future::err(e.into())),
        }
    }

    pub fn update(
        &self,
        check_run_id: &str,
        check_run_options: &CheckRunUpdateOptions,
    ) -> Future<CheckRun> {
        match serde_json::to_string(check_run_options) {
            Ok(data) => self.github.post_media::<CheckRun>(
                &self.path(&format!("/{}", check_run_id)),
                data.into_bytes(),
                MediaType::Preview("antiope"),
                AuthenticationConstraint::Unconstrained,
            ),
            Err(e) => Box::pin(futures::future::err(e.into())),
        }
    }

    pub fn list_for_suite(&self, suite_id: &str) -> Future<Vec<CheckRun>> {
        // !!! does this actually work?
        // https://developer.github.com/v3/checks/runs/#list-check-runs-in-a-check-suite
        self.github.get_media::<Vec<CheckRun>>(
            &self.path(&format!("/{}/check-runs", suite_id)),
            MediaType::Preview("antiope"),
        )
    }
}

// representations

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunState {
    Queued,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Conclusion {
    Success,
    Failure,
    Neutral,
    Cancelled,
    TimedOut,
    ActionRequired,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationLevel {
    Notice,
    Warning,
    Failure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub title: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub label: String,
    pub description: String,
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub path: String,
    pub start_line: u32,
    pub end_line: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<u32>,
    pub annotation_level: AnnotationLevel,
    pub message: String,
    pub title: String,
    pub raw_details: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub alt: String,
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CheckRunOptions {
    pub name: String,
    pub head_sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckRunState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Conclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CheckRunUpdateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckRunState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<Conclusion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CheckRun {
    pub id: i32,
    pub name: String,
    pub head_sha: String,
    pub url: String,
    pub check_suite: CheckSuite,
    pub details_url: Option<String>,
    pub external_id: Option<String>,
    pub status: Option<CheckRunState>,
    pub started_at: Option<String>,
    pub conclusion: Option<Conclusion>,
    pub completed_at: Option<String>,
    /*
    Deleted for now:

    GitHub's API returns:

      "output": {
        "title": null,
        "summary": null,
        "text": null,
        "annotations_count": 0,
        "annotations_url": "https://api.github.com/repos/grahamc/notpkgs/check-runs/30726963/annotations"
      },

    if there is no Output, which confuses serde.


    pub output: Option<Output>,
     */
    pub actions: Option<Vec<Action>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CheckSuiteResponse {
    #[serde(default, deserialize_with = "deserialize_null_u32::deserialize")]
    pub total_count: u32,
    #[serde(default)]
    pub check_suites: Vec<CheckSuite>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CheckSuite {
    pub id: u32,
    pub head_branch: String,
    pub head_sha: String,
    pub status: String,
    #[serde(default, deserialize_with = "deserialize_null_string::deserialize")]
    pub conclusion: String,
    #[serde(default)]
    pub app: CheckSuiteApp,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub mod deserialize_null_string {
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Sometimes this value is passed by the API as "null" which breaks the
        // std User parsing. We fix that here.
        let s = String::deserialize(deserializer).unwrap_or_default();

        Ok(s)
    }
}

pub mod deserialize_null_u32 {
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Sometimes this value is passed by the API as "null" which breaks the
        // std u32 parsing. We fix that here.
        let s = u32::deserialize(deserializer).unwrap_or(0);

        Ok(s)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CheckSuiteApp {
    pub id: u32,
    pub slug: String,
    pub name: String,
}

#[derive(Default)]
pub struct CheckSuiteListOptions {
    params: HashMap<&'static str, String>,
}

impl CheckSuiteListOptions {
    pub fn builder() -> CheckSuiteListOptionsBuilder {
        CheckSuiteListOptionsBuilder::default()
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
pub struct CheckSuiteListOptionsBuilder(CheckSuiteListOptions);

impl CheckSuiteListOptionsBuilder {
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

    pub fn build(&self) -> CheckSuiteListOptions {
        CheckSuiteListOptions {
            params: self.0.params.clone(),
        }
    }
}
