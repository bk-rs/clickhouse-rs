use std::error;

use super::helpers::*;

#[tokio::test]
async fn simple() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_client()?;

    client.ping().await?;

    Ok(())
}

#[tokio::test]
async fn with_anonymous() -> Result<(), Box<dyn error::Error>> {
    init_logger();

    let client = get_anonymous_client()?;

    client.ping().await?;

    Ok(())
}
