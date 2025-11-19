use anyhow::Result;
use core::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
