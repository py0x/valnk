use std::fmt;
use std::ops::Sub;
use serde::{Serialize, Deserialize};
use serde_dynamo;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

use super::entity::{EntityType, EntityId};
use super::submission::{SubmissionId, SUBMISSION_TAG};
use super::comment::{CommentId, COMMENT_TAG};

pub const REPLY_TAG: &str = "REPLY";
const AUTHOR_TAG: &str = "AUTHR";

pub type ReplyId = EntityId;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PrimaryKey {
    #[serde(rename(serialize = "PK", deserialize = "PK"))]
    pub pk: String,
    #[serde(rename(serialize = "SK", deserialize = "SK"))]
    pub sk: String,
}

/// The PrimaryKey of the `reply` item.
impl PrimaryKey {
    /// # Examples:
    ///
    /// ```
    /// use valnk::data::model::reply::{PrimaryKey, ReplyId};
    /// let id = ReplyId::from("id1".to_string()).unwrap();
    /// let pk = PrimaryKey::new(&id);
    ///
    /// assert_eq!(pk, PrimaryKey {
    ///     pk: String::from("REPLY#id1"),
    ///     sk: String::from("A"),
    /// });
    /// ```
    pub fn new(id: &ReplyId) -> Self {
        let id_str = id.as_ref();
        let pk = format!("{REPLY_TAG}#{id_str}");
        let sk = String::from("A");

        return Self {
            pk,
            sk,
        };
    }
}

/// For indexing comments by `submission_id` and `comment_id`.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SubmissionCommentIndexKey {
    #[serde(rename(serialize = "GSI1_PK", deserialize = "GSI1_PK"))]
    pub pk: String,
    #[serde(rename(serialize = "GSI1_SK", deserialize = "GSI1_SK"))]
    pub sk: String,
}

impl SubmissionCommentIndexKey {
    /// # Examples
    ///
    /// ```
    /// use valnk::data::model::submission::SubmissionId;
    /// use valnk::data::model::comment::CommentId;
    /// use valnk::data::model::reply::SubmissionCommentIndexKey;
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    ///
    /// let subm = SubmissionId::from("submission_id_123".to_string()).unwrap();
    /// let comm = CommentId::from("comment_id_123".to_string()).unwrap();
    /// let created_at = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    ///
    /// let subm_key = SubmissionCommentIndexKey::new(&subm, &comm, &created_at);
    /// let expected = SubmissionCommentIndexKey {
    ///     pk: String::from("SUBMS#submission_id_123"),
    ///     sk: String::from("REPLY#comment_id_123#0000001234"),
    /// };
    /// assert_eq!(subm_key, expected);
    /// ```
    pub fn new(submission_id: &SubmissionId, comment_id: &CommentId, created_at: &DateTime<Utc>) -> Self {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{SUBMISSION_TAG}#{submission_id}");
        let sk = format!("{REPLY_TAG}#{comment_id}#{created_at_ts:010}");

        return Self {
            pk,
            sk,
        };
    }
}

/// For indexing replys by `author_id`.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
    /// use valnk::data::model::reply::AuthorIndexKey;
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    ///
    /// let author_id = "py0x";
    /// let created_at = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    ///
    /// let author_key = AuthorIndexKey::new(author_id, &created_at);
    /// let expected = AuthorIndexKey {
    ///     pk: String::from("AUTHR#py0x"),
    ///     sk: String::from("REPLY#0000001234"),
    /// };
    ///
    /// assert_eq!(author_key, expected);
    /// ```
    pub fn new(author_id: &str, created_at: &DateTime<Utc>) -> Self {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{AUTHOR_TAG}#{author_id}");
        let sk = format!("{REPLY_TAG}#{created_at_ts:010}");

        return Self {
            pk,
            sk,
        };
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Reply {
    // index-key fields
    #[serde(flatten)]
    pub primary_key: PrimaryKey,
    #[serde(flatten)]
    pub submission_comment_key: SubmissionCommentIndexKey,
    #[serde(flatten)]
    pub author_key: AuthorIndexKey,

    // data fields
    pub entity_type: EntityType,

    pub id: ReplyId,
    pub submission_id: SubmissionId,
    pub comment_id: CommentId,
    pub author_id: String,
    pub text: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct ReplyBuilder {
    pub id: Option<ReplyId>,
    pub submission_id: Option<SubmissionId>,
    pub comment_id: Option<CommentId>,
    pub author_id: Option<String>,
    pub text: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Error, Debug)]
pub enum ReplyBuildError {
    #[error("the data for field `{0}` cannot be empty")]
    EmptyData(String),

    #[error("the data for field `{0}` is not valid, reason: `{1}`")]
    InvalidData(String, String),

    #[error("failed to build reply, reason: `{0}`")]
    Error(String),

    #[error("unknown reply build error")]
    Unknown,
}

impl ReplyBuilder {
    pub fn new() -> Self {
        return ReplyBuilder::default();
    }

    pub fn with_id(mut self, id: ReplyId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_submission_id(mut self, submission_id: SubmissionId) -> Self {
        self.submission_id = Some(submission_id);
        self
    }

    pub fn with_comment_id(mut self, comment_id: CommentId) -> Self {
        self.comment_id = Some(comment_id);
        self
    }

    pub fn with_author_id(mut self, author_id: String) -> Self {
        self.author_id = Some(author_id);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
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

    /// Build a `Reply` step by step
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    /// use valnk::data::model::entity::EntityType;
    /// use valnk::data::model::submission::SubmissionId;
    /// use valnk::data::model::comment::CommentId;
    /// use valnk::data::model::reply::*;
    ///
    /// let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    /// let reply_id = ReplyId::from("id111".to_string()).unwrap();
    /// let submission_id = SubmissionId::from("subm111".to_string()).unwrap();
    /// let comment_id = CommentId::from("commt111".to_string()).unwrap();
    ///
    /// let result = ReplyBuilder::new()
    ///     .with_id(reply_id.clone())
    ///     .with_submission_id(submission_id.clone())
    ///     .with_comment_id(comment_id.clone())
    ///     .with_author_id("author111".to_string())
    ///     .with_text("text111".to_string())
    ///     .with_created_at(current_dt)
    ///     .with_updated_at(current_dt)
    ///     .build()
    ///     .unwrap();
    ///
    /// let expected = Reply{
    ///     primary_key: PrimaryKey::new(&reply_id),
    ///     submission_comment_key: SubmissionCommentIndexKey::new(&submission_id, &comment_id, &current_dt),
    ///     author_key: AuthorIndexKey::new("author111", &current_dt),
    ///     entity_type: EntityType::Reply,
    ///     id: reply_id.clone(),
    ///     submission_id: submission_id.clone(),
    ///     comment_id: comment_id.clone(),
    ///     author_id: "author111".to_string(),
    ///     text: "text111".to_string(),
    ///     created_at: current_dt,
    ///     updated_at: current_dt,
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn build(self) -> Result<Reply, ReplyBuildError> {
        let id = self.id.unwrap_or(ReplyId::new());

        let submission_id = self.submission_id.ok_or(
            ReplyBuildError::EmptyData("submission_id".to_string())
        )?;

        let comment_id = self.comment_id.ok_or(
            ReplyBuildError::EmptyData("comment_id".to_string())
        )?;

        let author_id = self.author_id.ok_or(
            ReplyBuildError::EmptyData("author_id".to_string())
        )?;

        let text = self.text.ok_or(
            ReplyBuildError::EmptyData("text".to_string())
        )?;

        let current_dt = Utc::now();
        let created_at = self.created_at.unwrap_or(current_dt);
        let updated_at = self.updated_at.unwrap_or(current_dt);


        let primary_key = PrimaryKey::new(&id);
        let submission_comment_key = SubmissionCommentIndexKey::new(&submission_id, &comment_id, &created_at);
        let author_key = AuthorIndexKey::new(&author_id, &created_at);

        Ok(Reply {
            primary_key,
            submission_comment_key,
            author_key,
            entity_type: EntityType::Reply,
            id,
            submission_id,
            comment_id,
            author_id,
            text,
            created_at,
            updated_at,
        })
    }
}