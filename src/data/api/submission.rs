use aws_config;
use aws_sdk_dynamodb::Client as DynamodbClient;
use thiserror::Error;
use serde_dynamo;

use crate::data::model::submission::Submission;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("failed to create submission, invalid data: `{0}`")]
    InvalidData(String),

    #[error("failed to create submission, server error: `{0}`")]
    ServerError(String),
}

pub type Result<T, E = ClientError> = std::result::Result<T, E>;

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
            .map_err(|e| {
                ClientError::InvalidData(e.to_string())
            })?;

        self.ddb_cli
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| {
                ClientError::ServerError(e.to_string())
            })?;

        Ok(())
    }
}