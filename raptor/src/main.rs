use raptor::app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new().await?;
    app.run().await?;
    Ok(())
}
