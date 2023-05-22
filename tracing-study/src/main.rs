use log;
use tracing::{event, info, instrument, span, Level, Instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[instrument]
fn foo1(ans: i32) {
    info!("in foo");
}

async fn foo2() -> i32{
    // tokio sleep 1 sec
    info!("in foo2 1st log");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    info!("in foo2 2nd log");
    let ans = 42;
    ans 
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    log::info!("Hello world");

    let foo = 42;
    tracing::info!(foo, "Hello from tracing");

    event!(Level::INFO, "Somethign happened");

    let span = span!(Level::INFO, "my_span");
    {
        let _guard = span.enter();
        event!(Level::DEBUG, "something happened inside my_span");
    }

    foo1(42);

    tokio::spawn(async {
        foo2().instrument(tracing::info_span!("my_future")).await;
    }).await.unwrap();
}
