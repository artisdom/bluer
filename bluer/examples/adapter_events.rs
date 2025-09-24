//! Listen to Bluetooth adapter lifecycle events (adapter added/removed).

use bluer::{Session, SessionEvent};
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    env_logger::init();

    let session = Session::new().await?;

    println!("Listening for Bluetooth adapter events (adapter added/removed)");
    println!("Press Ctrl+C to quit\n");

    let events = session.events().await?;
    futures::pin_mut!(events);

    while let Some(event) = events.next().await {
        match event {
            SessionEvent::AdapterAdded(name) => {
                println!("Adapter added: {}", name);
            }
            SessionEvent::AdapterRemoved(name) => {
                println!("Adapter removed: {}", name);
            }
        }
    }

    Ok(())
}
