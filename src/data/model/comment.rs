use std::fmt;
use std::ops::Sub;
use serde::{Serialize, Deserialize};
use serde_dynamo;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

use super::entity::{EntityType, EntityId};
use super::submission::{SubmissionId, SUBMISSION_TAG};

pub const COMMENT_TAG: &str = "COMMT";
const AUTHOR_TAG: &str = "AUTHR";

pub type RankingScore = i64;
pub type CommentId = EntityId;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
    /// let id = CommentId::from("id1".to_string()).unwrap();
    /// let pk = PrimaryKey::new(&id);
    ///
    /// assert_eq!(pk, PrimaryKey {
    ///     pk: String::from("COMMT#id1"),
    ///     sk: String::from("A"),
    /// });
    /// ```
    pub fn new(id: &CommentId) -> PrimaryKey {
        let id_str = id.as_ref();
        let pk = format!("{COMMENT_TAG}#{id_str}");
        let sk = String::from("A");

        return PrimaryKey {
            pk,
            sk,
        };
    }
}

/// For indexing comments by `submission_id`.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
    /// let subm = SubmissionId::from("submission_id_123".to_string()).unwrap();
    /// let score = 192;
    ///
    /// let subm_key = SubmissionIndexKey::new(&subm, &score);
    /// let expected = SubmissionIndexKey {
    ///     pk: String::from("SUBMS#submission_id_123"),
    ///     sk: String::from("COMMT#0000000192"),
    /// };
    /// assert_eq!(topic_key, expected);
    /// ```
    pub fn new(submission_id: &SubmissionId, score: &RankingScore) -> SubmissionIndexKey {
        let pk = format!("{SUBMISSION_TAG}#{submission_id}");
        let sk = format!("{COMMENT_TAG}#{score:010}");

        return SubmissionIndexKey {
            pk,
            sk,
        };
    }
}

/// For indexing comments by `author_id`.
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
    ///     sk: String::from("COMMT#0000001234"),
    /// };
    ///
    /// assert_eq!(author_key, expected);
    /// ```
    pub fn new(author_id: &str, created_at: &DateTime<Utc>) -> AuthorIndexKey {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{AUTHOR_TAG}#{author_id}");
        let sk = format!("{COMMENT_TAG}#{created_at_ts:010}");

        return AuthorIndexKey {
            pk,
            sk,
        };
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    // todo

    // primary_key
    // submission_index_key


    // id
    // submission_id
    //entity type
    // author_id
    // ranking_score
    // text

    // n_likes, n_replys
    //
    //created_at,  // created an abstract datetime for entity?
    //updated_at,
}