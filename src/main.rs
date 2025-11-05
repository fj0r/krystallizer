use anyhow::Result;
use krystallizer::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
