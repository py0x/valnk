use std::collections::HashMap;
use super::submission;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

use tokio;

use serde_dynamo;
use aws_sdk_dynamodb;
use aws_config;

#[tokio::test]
async fn test_list_items_by_topic() {
    let shared_config = aws_config::load_from_env().await;
    let aws_cli = aws_sdk_dynamodb::Client::new(&shared_config);

    let cli = submission::Client::new(&aws_cli, "valnk-content");


    let mut input = submission::ListItemsByTopicInput::new("news");
    input.limit = Some(1);


    let output = cli.list_items_by_topic(input).await.unwrap();

    println!("output: {:#?}", output);
    println!("========");


    let mut input2 = submission::ListItemsByTopicInput::new("news");
    input2.limit = Some(2);
    input2.start_cursor = output.next_cursor;


    let output2 = cli.list_items_by_topic(input2).await.unwrap();

    println!("output2: {:#?}", output2);
    println!("========");

}