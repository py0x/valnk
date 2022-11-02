use std::fmt;
use serde::{Serialize, Deserialize};
use serde_dynamo;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

use super::entity::EntityType;

type RankingScore = u32;

const SUBMISSION_TAG: &str = "SUBMS";
const TOPIC_TAG: &str = "TOPIC";
const AUTHOR_TAG: &str = "AUTHR";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SubmissionId(String);

impl SubmissionId {
    pub fn new() -> SubmissionId {
        let id = Uuid::new_v4().to_string();
        return SubmissionId(id);
    }

    pub fn from(id: String) -> Result<SubmissionId, String> {
        if id.len() > 0 {
            return Ok(SubmissionId(id));
        }

        return Err("invalid submission id: empty id".to_string());
    }
}

impl AsRef<str> for SubmissionId {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl fmt::Display for SubmissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PrimaryKey {
    #[serde(rename(serialize = "PK", deserialize = "PK"))]
    pub pk: String,
    #[serde(rename(serialize = "SK", deserialize = "SK"))]
    pub sk: String,
}

/// The PrimaryKey of the `submission` item.
impl PrimaryKey {
    /// The PrimaryKey of submissions.
    ///
    /// # Examples:
    ///
    /// ```
    /// use valnk::ddb_data::submission::{PrimaryKey, SubmissionId};
    /// let id = SubmissionId::from("id1".to_string()).unwrap();
    /// let pk = PrimaryKey::new(&id);
    ///
    /// assert_eq!(pk, PrimaryKey {
    ///     pk: String::from("SUBMS#id1"),
    ///     sk: String::from("A"),
    /// });
    /// ```
    pub fn new(id: &SubmissionId) -> PrimaryKey {
        let id_str = id.as_ref();
        let pk = format!("{SUBMISSION_TAG}#{id_str}");
        let sk = String::from("A");

        return PrimaryKey {
            pk,
            sk,
        };
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TopicIndexKey {
    #[serde(rename(serialize = "GSI1_PK", deserialize = "GSI1_PK"))]
    pub pk: String,
    #[serde(rename(serialize = "GSI1_SK", deserialize = "GSI1_SK"))]
    pub sk: String,
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
    /// let topic_key = TopicIndexKey::new(topic, &score);
    /// let expected = TopicIndexKey {
    ///     pk: String::from("TOPIC#news"),
    ///     sk: String::from("SUBMS#0000000192"),
    /// };
    /// assert_eq!(topic_key, expected);
    /// ```
    pub fn new(topic: &str, score: &RankingScore) -> TopicIndexKey {
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
    pub fn new() -> SubmissionBuilder {
        return SubmissionBuilder::default();
    }

    pub fn with_id(mut self, id: SubmissionId) -> SubmissionBuilder {
        self.id = Some(id);
        self
    }

    pub fn with_author_id(mut self, author_id: String) -> SubmissionBuilder {
        self.author_id = Some(author_id);
        self
    }

    pub fn with_topic(mut self, topic: String) -> SubmissionBuilder {
        self.topic = Some(topic);
        self
    }

    pub fn with_ranking_score(mut self, ranking_score: RankingScore) -> SubmissionBuilder {
        self.ranking_score = Some(ranking_score);
        self
    }

    pub fn with_title(mut self, title: String) -> SubmissionBuilder {
        self.title = Some(title);
        self
    }

    pub fn with_url(mut self, url: String) -> SubmissionBuilder {
        self.url = Some(url);
        self
    }

    pub fn with_text(mut self, text: String) -> SubmissionBuilder {
        self.text = Some(text);
        self
    }

    pub fn with_created_at(mut self, created_at: DateTime<Utc>) -> SubmissionBuilder {
        self.created_at = Some(created_at);
        self
    }

    pub fn with_updated_at(mut self, updated_at: DateTime<Utc>) -> SubmissionBuilder {
        self.updated_at = Some(updated_at);
        self
    }

    /// Build a `Submission` step by step
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    /// use valnk::ddb_data::entity::EntityType;
    /// use valnk::ddb_data::submission::{AuthorIndexKey, PrimaryKey, Submission, SubmissionBuilder, SubmissionId, TopicIndexKey};
    ///
    /// let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
    /// let result = SubmissionBuilder::new()
    ///     .with_id(SubmissionId::from("id111".to_string()).unwrap())
    ///     .with_author_id("author111".to_string())
    ///     .with_topic("topic111".to_string())
    ///     .with_ranking_score(999)
    ///     .with_title("title111".to_string())
    ///     .with_url("url111".to_string())
    ///     .with_text("text111".to_string())
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
    ///     created_at: current_dt,
    ///     updated_at: current_dt,
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn build(self) -> Result<Submission, SubmissionBuildError> {
        let id = match self.id {
            None => SubmissionId::new(),
            Some(id0) => id0,
        };

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
            created_at,
            updated_at,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::entity::EntityType;
    use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

    #[test]
    fn test_submission_builder() {
        let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234,0), Utc);
        let result = SubmissionBuilder::new()
            // .with_id(SubmissionId::from("id111".to_string()).unwrap())
            .with_author_id("author111".to_string())
            .with_topic("topic111".to_string())
            .with_ranking_score(999)
            .with_title("title111".to_string())
            .with_url("url111".to_string())
            .with_text("text111".to_string())
            // .with_created_at(current_dt)
            // .with_updated_at(current_dt)
            .build()
            .unwrap();

        println!("{result:#?}");
    }
}