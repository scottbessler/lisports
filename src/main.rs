#[tokio::main]
async fn main() {
    if let Err(err) = lisports::app::run().await {
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}
