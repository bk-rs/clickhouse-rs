use std::error;

use chrono::{NaiveDateTime, Utc};
use clickhouse_http_client::clickhouse_format::{
    input::JsonCompactEachRowInput, output::JsonCompactEachRowWithNamesAndTypesOutput,
};
use serde::Deserialize;
use serde_json::Value;

use super::helpers::*;

#[derive(Deserialize, Debug)]
struct Event {
    #[serde(rename = "event_id")]
    id: u32,
    #[serde(deserialize_with = "clickhouse_data_value::datetime::deserialize")]
    created_at: NaiveDateTime,
}

#[tokio::test]
async fn simple() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    client
        .execute(
            r#"
CREATE TABLE t_testing_events
(
    event_id UInt32,
    created_at Datetime('UTC')
) ENGINE=Memory
            "#,
            None,
        )
        .await?;

    let rows: Vec<Vec<Value>> = vec![
        vec![1.into(), Utc::now().timestamp().into()],
        vec![2.into(), Utc::now().timestamp().into()],
    ];
    client
        .insert_with_format(
            "INSERT INTO t_testing_events (event_id, created_at)",
            JsonCompactEachRowInput::new(rows),
            None,
        )
        .await?;

    let (events, info) = client
        .select_with_format(
            "SELECT * FROM t_testing_events",
            JsonCompactEachRowWithNamesAndTypesOutput::<Event>::new(),
            None,
        )
        .await?;
    println!("{:?}", events);
    println!("{:?}", info);
    assert_eq!(events.len(), 2);
    let event = events.first().unwrap();
    assert_eq!(event.id, 1);

    let (events, info) = client
        .select_with_format(
            "SELECT * FROM t_testing_events",
            JsonCompactEachRowWithNamesAndTypesOutput::<Event>::new(),
            vec![("date_time_output_format", "iso")],
        )
        .await?;
    println!("{:?}", events);
    println!("{:?}", info);

    client.execute("DROP TABLE t_testing_events", None).await?;

    Ok(())
}
