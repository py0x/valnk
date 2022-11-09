use std::fmt;
use serde::{Serialize, Deserialize};
use serde_dynamo;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

use super::entity::{EntityType, EntityId};

pub const SUBMISSION_TAG: &str = "SUBMS";
const TOPIC_TAG: &str = "TOPIC";
const AUTHOR_TAG: &str = "AUTHR";

pub type RankingScore = i64;
pub type SubmissionId = EntityId;

/// The PrimaryKey of the `submission` item.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PrimaryKey {
    #[serde(rename(serialize = "PK", deserialize = "PK"))]
    pub pk: String,
    #[serde(rename(serialize = "SK", deserialize = "SK"))]
    pub sk: String,
}

impl PrimaryKey {
    /// # Examples:
    ///
    /// ```
    /// use valnk::data::model::submission::{PrimaryKey, SubmissionId};
    /// let id = SubmissionId::from("id1".to_string()).unwrap();
    /// let pk = PrimaryKey::new(&id);
    ///
    /// assert_eq!(pk, PrimaryKey {
    ///     pk: String::from("SUBMS#id1"),
    ///     sk: String::from("A"),
    /// });
    /// ```
    pub fn new(id: &SubmissionId) -> Self {
        let id_str = id.as_ref();
        let pk = format!("{SUBMISSION_TAG}#{id_str}");
        let sk = String::from("A");

        return Self {
            pk,
            sk,
        };
    }
}


/// For indexing submissions by `topic`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TopicIndexKey {
    #[serde(rename(serialize = "GSI1_PK", deserialize = "GSI1_PK"))]
    pub pk: String,
    #[serde(rename(serialize = "GSI1_SK", deserialize = "GSI1_SK"))]
    pub sk: String,
}

impl TopicIndexKey {
    pub const INDEX_NAME: &'static str = "GSI1";

    /// # Examples
    ///
    /// ```
    /// use valnk::data::model::submission::TopicIndexKey;
    ///
    /// let topic = "topic_xxx";
    /// let score = 192;
    ///
    /// let topic_key = TopicIndexKey::new(topic, &score);
    /// let expected = TopicIndexKey {
    ///     pk: String::from("TOPIC#topic_xxx"),
    ///     sk: String::from("SUBMS#0000000192"),
    /// };
    /// assert_eq!(topic_key, expected);
    /// ```
    pub fn new(topic: &str, score: &RankingScore) -> Self {
        return Self {
            pk: Self::pk(topic),
            sk: Self::sk(score),
        };
    }

    pub fn pk(topic: &str) -> String {
        format!("{TOPIC_TAG}#{topic}")
    }

    pub fn sk(score: &RankingScore) -> String {
        let pfx = Self::sk_prefix();
        return format!("{pfx}{score:010}");
    }

    pub fn sk_prefix() -> String {
        return format!("{SUBMISSION_TAG}#");
    }
}

/// For indexing submissions by `author_id`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AuthorIndexKey {
    #[serde(rename(serialize = "GSI2_PK", deserialize = "GSI2_PK"))]
    pub pk: String,
    #[serde(rename(serialize = "GSI2_SK", deserialize = "GSI2_SK"))]
    pub sk: String,
}

impl AuthorIndexKey {
    /// # Examples
    ///
    /// ```
    /// use valnk::data::model::submission::AuthorIndexKey;
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    ///
    /// let author_id = "py0x";
    /// let created_at = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    ///
    /// let author_key = AuthorIndexKey::new(author_id, &created_at);
    /// let expected = AuthorIndexKey {
    ///     pk: String::from("AUTHR#py0x"),
    ///     sk: String::from("SUBMS#0000001234"),
    /// };
    ///
    /// assert_eq!(author_key, expected);
    /// ```
    pub fn new(author_id: &str, created_at: &DateTime<Utc>) -> Self {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{AUTHOR_TAG}#{author_id}");
        let sk = format!("{SUBMISSION_TAG}#{created_at_ts:010}");

        return Self {
            pk,
            sk,
        };
    }
}


#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Submission {
    // index-key fields
    #[serde(flatten)]
    pub primary_key: PrimaryKey,
    #[serde(flatten)]
    pub topic_key: TopicIndexKey,
    #[serde(flatten)]
    pub author_key: AuthorIndexKey,

    // data fields
    pub entity_type: EntityType,

    pub id: SubmissionId,
    pub author_id: String,
    pub topic: String,
    pub ranking_score: RankingScore,
    pub title: String,
    pub url: String,
    pub text: String,

    pub n_votes: u64,
    pub n_comments: u64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct SubmissionBuilder {
    id: Option<SubmissionId>,
    author_id: Option<String>,
    topic: Option<String>,
    ranking_score: Option<RankingScore>,
    title: Option<String>,
    url: Option<String>,
    text: Option<String>,

    n_votes: Option<u64>,
    n_comments: Option<u64>,

    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Error, Debug)]
pub enum SubmissionBuildError {
    #[error("the data for field `{0}` cannot be empty")]
    EmptyData(String),

    #[error("the data for field `{0}` is not valid, reason: `{1}`")]
    InvalidData(String, String),

    #[error("failed to build submission, reason: `{0}`")]
    Error(String),

    #[error("unknown submission build error")]
    Unknown,
}

impl SubmissionBuilder {
    pub fn new() -> Self {
        return SubmissionBuilder::default();
    }

    pub fn with_id(mut self, id: SubmissionId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_author_id(mut self, author_id: impl Into<String>) -> Self {
        self.author_id = Some(author_id.into());
        self
    }

    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn with_ranking_score(mut self, ranking_score: RankingScore) -> Self {
        self.ranking_score = Some(ranking_score);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_n_votes(mut self, n_votes: u64) -> Self {
        self.n_votes = Some(n_votes);
        self
    }

    pub fn with_n_comments(mut self, n_comments: u64) -> Self {
        self.n_comments = Some(n_comments);
        self
    }

    pub fn with_created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn with_updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    /// Build a `Submission` step by step
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    /// use valnk::data::model::entity::EntityType;
    /// use valnk::data::model::submission::*;
    ///
    /// let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    /// let result = SubmissionBuilder::new()
    ///     .with_id(SubmissionId::from("id111").unwrap())
    ///     .with_author_id("author111")
    ///     .with_topic("topic111")
    ///     .with_ranking_score(999)
    ///     .with_title("title111")
    ///     .with_url("url111")
    ///     .with_text("text111")
    ///     .with_created_at(current_dt)
    ///     .with_updated_at(current_dt)
    ///     .build()
    ///     .unwrap();
    ///
    /// let expected = Submission{
    ///     primary_key: PrimaryKey::new(&SubmissionId::from("id111".to_string()).unwrap()),
    ///     topic_key: TopicIndexKey::new("topic111", &999),
    ///     author_key: AuthorIndexKey::new("author111", &current_dt),
    ///     entity_type: EntityType::Submission,
    ///
    ///     id: SubmissionId::from("id111".to_string()).unwrap(),
    ///     author_id: "author111".to_string(),
    ///     topic: "topic111".to_string(),
    ///     ranking_score: 999,
    ///     title: "title111".to_string(),
    ///     url: "url111".to_string(),
    ///     text: "text111".to_string(),
    ///     n_votes: 0,
    ///     n_comments: 0,
    ///     created_at: current_dt,
    ///     updated_at: current_dt,
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn build(self) -> Result<Submission, SubmissionBuildError> {
        let id = self.id.unwrap_or(SubmissionId::new());

        let author_id = self.author_id.ok_or(
            SubmissionBuildError::EmptyData("author_id".to_string())
        )?;

        let topic = self.topic.ok_or(
            SubmissionBuildError::EmptyData("topic".to_string())
        )?;

        let ranking_score = self.ranking_score.ok_or(
            SubmissionBuildError::EmptyData("ranking_score".to_string())
        )?;

        let title = self.title.ok_or(
            SubmissionBuildError::EmptyData("title".to_string())
        )?;

        let url = self.url.ok_or(
            SubmissionBuildError::EmptyData("url".to_string())
        )?;

        let text = self.text.ok_or(
            SubmissionBuildError::EmptyData("text".to_string())
        )?;

        let n_votes = self.n_votes.unwrap_or(0);
        let n_comments = self.n_comments.unwrap_or(0);

        let current_dt = Utc::now();
        let created_at = self.created_at.unwrap_or(current_dt);
        let updated_at = self.updated_at.unwrap_or(current_dt);


        let primary_key = PrimaryKey::new(&id);
        let topic_key = TopicIndexKey::new(&topic, &ranking_score);
        let author_key = AuthorIndexKey::new(&author_id, &created_at);

        Ok(Submission {
            primary_key,
            topic_key,
            author_key,
            entity_type: EntityType::Submission,
            id,
            author_id,
            topic,
            ranking_score,
            title,
            url,
            text,
            n_votes,
            n_comments,
            created_at,
            updated_at,
        })
    }
}