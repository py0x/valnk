use serde::{Serialize, Deserialize};
use serde_dynamo;

use aws_config;
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::types::SdkError;


use crate::data::model::submission::{
    Submission,
    SUBMISSION_TAG,
    TopicIndexKey,
};

use super::result::{Error, Result};
use super::cursor::Cursor;


#[derive(Clone, Debug)]
pub struct ListItemsByTopicInput {
    pub topic: String,
    pub limit: Option<i32>,
    pub reverse: Option<bool>,
    pub start_cursor: Option<Cursor>,
}

impl ListItemsByTopicInput {
    pub fn new(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            limit: None,
            reverse: None,
            start_cursor: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ListItemsByTopicOutput {
    pub items: Vec<Submission>,
    pub next_cursor: Option<Cursor>,
}

impl ListItemsByTopicOutput {
    pub fn new(items: Vec<Submission>) -> Self {
        Self {
            items,
            next_cursor: None,
        }
    }
}


#[derive(Debug)]
pub struct Client<'c> {
    ddb_cli: &'c DynamodbClient,
    table_name: String,
}

impl<'c> Client<'c> {
    pub fn new(ddb_cli: &'c DynamodbClient, table_name: impl Into<String>) -> Self {
        return Self {
            ddb_cli,
            table_name: table_name.into(),
        };
    }

    /// # Example:
    ///
    /// ```no_run
    /// use tokio;
    /// use valnk::data::api::submission::*;
    /// use valnk::data::model::submission as subm_model;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shared_config = aws_config::load_from_env().await;
    ///     let aws_cli = aws_sdk_dynamodb::Client::new(&shared_config);
    ///     let cli = Client::new(&aws_cli, "valnk-content");
    ///     let subm = subm_model::SubmissionBuilder::new()
    ///         .with_author_id("py0x")
    ///         .with_topic("news")
    ///         .with_title("create_item test example")
    ///         .with_text("hello create_item")
    ///         .with_url("")
    ///         .with_ranking_score(10)
    ///         .build()
    ///         .unwrap();
    ///
    ///     cli.create_item(subm).await.unwrap();
    /// }
    /// ```
    pub async fn create_item(&self, subm: Submission) -> Result<()> {
        let item = serde_dynamo::to_item(subm)
            .map_err(Error::InvalidInputData)?;

        self.ddb_cli
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| Error::ServerError(e.to_string()))?;

        Ok(())
    }

    /// # Example:
    ///
    /// ```no_run
    /// use tokio;
    /// use valnk::data::api::submission::*;
    /// use valnk::data::model::submission as subm_model;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shared_config = aws_config::load_from_env().await;
    ///     let aws_cli = aws_sdk_dynamodb::Client::new(&shared_config);
    ///     let cli = Client::new(&aws_cli, "valnk-content");
    ///
    ///     let mut input = ListItemsByTopicInput::new("my-topic");
    ///     input.limit = Some(10);
    ///
    ///     let output = cli.list_items_by_topic(input).await.unwrap();
    /// }
    /// ```
    pub async fn list_items_by_topic(&self, input: ListItemsByTopicInput) -> Result<ListItemsByTopicOutput> {
        let mut limit = 30;
        let mut reverse = false;
        let mut exclusive_start_key = None;

        if let Some(lm) = input.limit {
            limit = lm;
        }

        if let Some(rv) = input.reverse {
            reverse = rv;
        }

        if let Some(cur) = input.start_cursor {
            let lk = cur.try_into()
                .map_err(Error::InvalidInputData)?;
            exclusive_start_key = Some(lk);
        }

        // more about `ddb_cli.query`:
        // https://docs.rs/aws-sdk-dynamodb/0.21.0/aws_sdk_dynamodb/client/struct.Client.html#method.query
        let results = self.ddb_cli
            .query()
            .table_name(&self.table_name)
            .index_name(TopicIndexKey::INDEX_NAME)
            .key_condition_expression("GSI1_PK = :topic_pk and begins_with(GSI1_SK, :tag_pfx)")
            .expression_attribute_values(
                ":topic_pk", AttributeValue::S(TopicIndexKey::pk(&input.topic)),
            )
            .expression_attribute_values(
                ":tag_pfx", AttributeValue::S(TopicIndexKey::sk_prefix()),
            )
            .scan_index_forward(reverse)
            .limit(limit)
            .set_exclusive_start_key(exclusive_start_key)
            .send()
            .await
            .map_err(|e| Error::ServerError(e.to_string()))?;


        let mut subms: Vec<Submission> = vec![];
        if let Some(items) = results.items() {
            subms = serde_dynamo::from_items(items.to_vec())
                .map_err(Error::InvalidOutputData)?;
        }
        let mut output = ListItemsByTopicOutput::new(subms);


        if let Some(lk) = results.last_evaluated_key() {
            let next_cursor = Cursor::try_from(lk.to_owned())
                .map_err(Error::InvalidOutputData)?;

            output.next_cursor = Some(next_cursor);
        }

        Ok(output)
    }

    // pub async fn get_items_by_author_id() -> Result<()> {}
}