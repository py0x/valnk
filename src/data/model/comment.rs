use std::fmt;
use std::ops::Sub;
use serde::{Serialize, Deserialize};
use serde_dynamo;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;
use crate::data::model::reply::SubmissionCommentIndexKey;

use super::entity::{EntityType, EntityId};
use super::submission::{SubmissionId, SUBMISSION_TAG};

pub const COMMENT_TAG: &str = "COMMT";
const AUTHOR_TAG: &str = "AUTHR";

pub type RankingScore = i64;
pub type CommentId = EntityId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PrimaryKey {
    #[serde(rename(serialize = "PK", deserialize = "PK"))]
    pub pk: String,
    #[serde(rename(serialize = "SK", deserialize = "SK"))]
    pub sk: String,
}

/// The PrimaryKey of the `comment` item.
impl PrimaryKey {
    /// # Examples:
    ///
    /// ```
    /// use valnk::data::model::comment::{PrimaryKey, CommentId};
    /// let id = CommentId::from("id1").unwrap();
    /// let pk = PrimaryKey::new(&id);
    ///
    /// assert_eq!(pk, PrimaryKey {
    ///     pk: String::from("COMMT#id1"),
    ///     sk: String::from("A"),
    /// });
    /// ```
    pub fn new(id: &CommentId) -> Self {
        let id_str = id.as_ref();
        let pk = format!("{COMMENT_TAG}#{id_str}");
        let sk = String::from("A");

        return Self {
            pk,
            sk,
        };
    }
}

/// For indexing comments by `submission_id`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct SubmissionIndexKey {
    #[serde(rename(serialize = "GSI1_PK", deserialize = "GSI1_PK"))]
    pub pk: String,
    #[serde(rename(serialize = "GSI1_SK", deserialize = "GSI1_SK"))]
    pub sk: String,
}

impl SubmissionIndexKey {
    /// # Examples
    ///
    /// ```
    /// use valnk::data::model::submission::SubmissionId;
    /// use valnk::data::model::comment::SubmissionIndexKey;
    ///
    /// let subm = SubmissionId::from("submission_id_123").unwrap();
    /// let score = 192;
    ///
    /// let subm_key = SubmissionIndexKey::new(&subm, &score);
    /// let expected = SubmissionIndexKey {
    ///     pk: String::from("SUBMS#submission_id_123"),
    ///     sk: String::from("COMMT#0000000192"),
    /// };
    /// assert_eq!(subm_key, expected);
    /// ```
    pub fn new(submission_id: &SubmissionId, score: &RankingScore) -> Self {
        let pk = format!("{SUBMISSION_TAG}#{submission_id}");
        let sk = format!("{COMMENT_TAG}#{score:010}");

        return Self {
            pk,
            sk,
        };
    }
}

/// For indexing comments by `author_id`.
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
    /// use valnk::data::model::comment::AuthorIndexKey;
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    ///
    /// let author_id = "py0x";
    /// let created_at = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    ///
    /// let author_key = AuthorIndexKey::new(author_id, &created_at);
    /// let expected = AuthorIndexKey {
    ///     pk: String::from("AUTHR#py0x"),
    ///     sk: String::from("COMMT#0000001234"),
    /// };
    ///
    /// assert_eq!(author_key, expected);
    /// ```
    pub fn new(author_id: &str, created_at: &DateTime<Utc>) -> Self {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{AUTHOR_TAG}#{author_id}");
        let sk = format!("{COMMENT_TAG}#{created_at_ts:010}");

        return Self {
            pk,
            sk,
        };
    }
}


#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Comment {
    // index-key fields
    #[serde(flatten)]
    pub primary_key: PrimaryKey,
    #[serde(flatten)]
    pub submission_key: SubmissionIndexKey,
    #[serde(flatten)]
    pub author_key: AuthorIndexKey,

    // data fields
    pub entity_type: EntityType,

    pub id: CommentId,
    pub submission_id: SubmissionId,
    pub author_id: String,
    pub ranking_score: RankingScore,
    pub text: String,

    pub n_votes: u64,
    pub n_replies: u64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct CommentBuilder {
    id: Option<CommentId>,
    submission_id: Option<SubmissionId>,
    author_id: Option<String>,
    topic: Option<String>,
    ranking_score: Option<RankingScore>,
    text: Option<String>,

    n_votes: Option<u64>,
    n_replies: Option<u64>,

    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Error, Debug)]
pub enum CommentBuildError {
    #[error("the data for field `{0}` cannot be empty")]
    EmptyData(String),

    #[error("the data for field `{0}` is not valid, reason: `{1}`")]
    InvalidData(String, String),

    #[error("failed to build comment, reason: `{0}`")]
    Error(String),

    #[error("unknown comment build error")]
    Unknown,
}

impl CommentBuilder {
    pub fn new() -> Self {
        return CommentBuilder::default();
    }

    pub fn with_id(mut self, id: CommentId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_submission_id(mut self, submission_id: SubmissionId) -> Self {
        self.submission_id = Some(submission_id);
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

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_n_votes(mut self, n_votes: u64) -> Self {
        self.n_votes = Some(n_votes);
        self
    }

    pub fn with_n_replies(mut self, n_replies: u64) -> Self {
        self.n_replies = Some(n_replies);
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

    /// Build a `Comment` step by step
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    /// use valnk::data::model::entity::EntityType;
    /// use valnk::data::model::submission::SubmissionId;
    /// use valnk::data::model::comment::*;
    ///
    /// let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    /// let result = CommentBuilder::new()
    ///     .with_id(CommentId::from("id111").unwrap())
    ///     .with_submission_id(SubmissionId::from("subm111").unwrap())
    ///     .with_author_id("author111")
    ///     .with_ranking_score(999)
    ///     .with_text("text111")
    ///     .with_created_at(current_dt)
    ///     .with_updated_at(current_dt)
    ///     .build()
    ///     .unwrap();
    ///
    /// let submission_id = SubmissionId::from("subm111").unwrap();
    /// let expected = Comment{
    ///     primary_key: PrimaryKey::new(&CommentId::from("id111").unwrap()),
    ///     submission_key: SubmissionIndexKey::new(&submission_id, &999),
    ///     author_key: AuthorIndexKey::new("author111", &current_dt),
    ///     entity_type: EntityType::Comment,
    ///     id: CommentId::from("id111").unwrap(),
    ///     submission_id: submission_id.clone(),
    ///     author_id: "author111".to_string(),
    ///     ranking_score: 999,
    ///     text: "text111".to_string(),
    ///     n_votes: 0,
    ///     n_replies: 0,
    ///     created_at: current_dt,
    ///     updated_at: current_dt,
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn build(self) -> Result<Comment, CommentBuildError> {
        let id = self.id.unwrap_or(CommentId::new());

        let submission_id = self.submission_id.ok_or(
            CommentBuildError::EmptyData("submission_id".to_string())
        )?;

        let author_id = self.author_id.ok_or(
            CommentBuildError::EmptyData("author_id".to_string())
        )?;

        let ranking_score = self.ranking_score.ok_or(
            CommentBuildError::EmptyData("ranking_score".to_string())
        )?;


        let text = self.text.ok_or(
            CommentBuildError::EmptyData("text".to_string())
        )?;

        let n_votes = self.n_votes.unwrap_or(0);
        let n_replies = self.n_replies.unwrap_or(0);

        let current_dt = Utc::now();
        let created_at = self.created_at.unwrap_or(current_dt);
        let updated_at = self.updated_at.unwrap_or(current_dt);


        let primary_key = PrimaryKey::new(&id);
        let submission_key = SubmissionIndexKey::new(&submission_id, &ranking_score);
        let author_key = AuthorIndexKey::new(&author_id, &created_at);

        Ok(Comment {
            primary_key,
            submission_key,
            author_key,
            entity_type: EntityType::Comment,
            id,
            submission_id,
            author_id,
            ranking_score,
            text,
            n_votes,
            n_replies,
            created_at,
            updated_at,
        })
    }
}