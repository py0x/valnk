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
    pub fn new(id: &ReplyId) -> PrimaryKey {
        let id_str = id.as_ref();
        let pk = format!("{REPLY_TAG}#{id_str}");
        let sk = String::from("A");

        return PrimaryKey {
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
    /// assert_eq!(topic_key, expected);
    /// ```

    pub fn new(submission_id: &SubmissionId, comment_id: &CommentId, created_at: &DateTime<Utc>) -> SubmissionCommentIndexKey {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{SUBMISSION_TAG}#{submission_id}");
        let sk = format!("{REPLY_TAG}#{comment_id}#{created_at_ts:010}");

        return SubmissionCommentIndexKey {
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
    /// use valnk::data::model::comment::AuthorIndexKey;
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
    pub fn new(author_id: &str, created_at: &DateTime<Utc>) -> AuthorIndexKey {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{AUTHOR_TAG}#{author_id}");
        let sk = format!("{REPLY_TAG}#{created_at_ts:010}");

        return AuthorIndexKey {
            pk,
            sk,
        };
    }
}