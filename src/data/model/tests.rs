use std::collections::HashMap;
use super::submission::*;
use super::entity::EntityType;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

use tokio;

use serde_dynamo;
use aws_sdk_dynamodb;
use aws_config;
use serde_dynamo::Item;

// #[test]
// fn test_submission_builder() {
//     let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234, 0), Utc);
//     let result = SubmissionBuilder::new()
//         // .with_id(SubmissionId::from("id111".to_string()).unwrap())
//         .with_author_id("author111".to_string())
//         .with_topic("topic111".to_string())
//         .with_ranking_score(999)
//         .with_title("title111".to_string())
//         .with_url("url111".to_string())
//         .with_text("text111".to_string())
//         // .with_created_at(current_dt)
//         // .with_updated_at(current_dt)
//         .build()
//         .unwrap();
//
//     // println!("{result:#?}");
//
//     let item: Item = serde_dynamo::to_item(result).unwrap();
//
//     println!("{item:#?}");
// }

#[tokio::test]
async fn test_put_item() {
    let current_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1234, 0), Utc);
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

    // println!("{result:#?}");

    let item = serde_dynamo::to_item(result).unwrap();

    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);

    let res = client
        .put_item()
        .table_name("valnk-content")
        .set_item(Some(item))
        .send()
        .await.unwrap();

    println!("{res:#?}");
}

#[tokio::test]
async fn test_scan_item() {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);

    // Get documents from DynamoDB
    let result = client.scan().table_name("valnk-content").send().await.unwrap();

    for item in result.items.unwrap() {
        let subm: Submission = serde_dynamo::from_item(item).unwrap();
        println!("subm: {subm:#?}");
    }
}