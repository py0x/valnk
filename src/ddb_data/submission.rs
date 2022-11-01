use serde::{Serialize, Deserialize};
use serde_dynamo;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::entity::EntityType;

type RankingScore = u32;

const SUBMISSION_TAG: &str = "SUBMS";
const TOPIC_TAG: &str = "TOPIC";
const AUTHOR_TAG: &str = "AUTHR";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PrimaryKey {
    #[serde(rename(serialize = "PK", deserialize = "PK"))]
    pub pk: String,
    #[serde(rename(serialize = "SK", deserialize = "SK"))]
    pub sk: String,
}

/// The PrimaryKey for the `submission` item.
///
/// # Examples
///
/// ```
/// use valnk::ddb_data::submission::PrimaryKey;
///
/// let r1 = PrimaryKey::generate();
/// let r1_id = str::replace(&r1.pk, "SUBMS#", "");
/// let r2 = PrimaryKey::new(&r1_id);
///
/// assert_eq!(r1, r2);
/// assert_eq!(&r1.sk, "A");
/// ```
impl PrimaryKey {
    /// The PrimaryKey of submissions.
    ///
    /// # Examples:
    ///
    /// ```
    /// use valnk::ddb_data::submission::PrimaryKey;
    /// let pk = PrimaryKey::new("id1");
    ///
    /// assert_eq!(pk, PrimaryKey {
    ///     pk: String::from("SUBMS#id1"),
    ///     sk: String::from("A"),
    /// });
    /// ```
    pub fn new(id: &str) -> PrimaryKey {
        let pk = format!("{SUBMISSION_TAG}#{id}");
        let sk = String::from("A");

        return PrimaryKey {
            pk,
            sk,
        };
    }

    pub fn generate() -> PrimaryKey {
        let id = Uuid::new_v4().to_string();
        return PrimaryKey::new(&id);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TopicIndexKey {
    #[serde(rename(serialize = "GSI1_PK", deserialize = "GSI1_PK"))]
    pub pk: String,
    // "TOPIC#{topic}"
    #[serde(rename(serialize = "GSI1_SK", deserialize = "GSI1_SK"))]
    pub sk: String,  // "SUBMS#{ranking_score}"
}

impl TopicIndexKey {
    /// For indexing submissions by `topic`.
    ///
    /// # Examples
    ///
    /// ```
    /// use valnk::ddb_data::submission::TopicIndexKey;
    ///
    /// let topic = "news";
    /// let score = 192;
    ///
    /// let topic_key = TopicIndexKey::new(topic, score);
    /// let expected = TopicIndexKey {
    ///     pk: String::from("TOPIC#news"),
    ///     sk: String::from("SUBMS#0000000192"),
    /// };
    /// assert_eq!(topic_key, expected);
    /// ```
    pub fn new(topic: &str, score: RankingScore) -> TopicIndexKey {
        let pk = format!("{TOPIC_TAG}#{topic}");
        let sk = format!("{SUBMISSION_TAG}#{score:010}");

        return TopicIndexKey {
            pk,
            sk,
        };
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AuthorIndexKey {
    #[serde(rename(serialize = "GSI2_PK", deserialize = "GSI2_PK"))]
    pub pk: String,
    #[serde(rename(serialize = "GSI2_SK", deserialize = "GSI2_SK"))]
    pub sk: String,
}

impl AuthorIndexKey {
    /// For indexing submissions by `author_id`.
    ///
    /// # Examples
    ///
    /// ```
    /// use valnk::ddb_data::submission::AuthorIndexKey;
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
    pub fn new(author_id: &str, created_at: &DateTime<Utc>) -> AuthorIndexKey {
        let created_at_ts = created_at.timestamp();

        let pk = format!("{AUTHOR_TAG}#{author_id}");
        let sk = format!("{SUBMISSION_TAG}#{created_at_ts:010}");

        return AuthorIndexKey {
            pk,
            sk,
        };
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Submission {
    // index fields
    #[serde(flatten)]
    primary_key: PrimaryKey,
    #[serde(flatten)]
    topic_key: TopicIndexKey,
    #[serde(flatten)]
    author_key: AuthorIndexKey,

    // data fields
    entity_type: EntityType,

    id: String,
    author_id: String,
    topic: String,
    ranking_score: RankingScore,
    title: String,
    url: String,
    text: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_primary_key() {
        let pk = PrimaryKey::new("id1");
        assert_eq!(pk, PrimaryKey {
            pk: String::from("SUBMS#id1"),
            sk: String::from("A"),
        })
    }
}