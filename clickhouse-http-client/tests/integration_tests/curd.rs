use std::error;

use super::helpers::*;

#[tokio::test]
async fn simple() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    client
        .execute(
            r#"
CREATE TABLE t_testing_http
(
    a UInt8
) ENGINE=Memory
            "#,
            None,
        )
        .await?;

    client.execute("DROP TABLE t_testing_http", None).await?;

    Ok(())
}
