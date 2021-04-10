use std::error;

use clickhouse_format::{
    input::JsonCompactEachRowInput, output::JsonCompactEachRowWithNamesAndTypesOutput,
};
use serde::Deserialize;
use serde_json::Value;

use super::helpers::*;

#[derive(Deserialize, Debug)]
struct Event {
    #[serde(rename = "event_id")]
    id: u32,
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
    event_id UInt32
) ENGINE=Memory
            "#,
            None,
        )
        .await?;

    let rows: Vec<Vec<Value>> = vec![vec![1.into()], vec![2.into()]];
    client
        .insert_with_format(
            "INSERT INTO t_testing_events",
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

    client.execute("DROP TABLE t_testing_events", None).await?;

    Ok(())
}
