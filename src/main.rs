// Clippy Rules
#![deny(clippy::unwrap_used)]
#![allow(clippy::single_char_pattern)]

use etcd_client::Error;
use veritas::db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = db::ConnectionManager::new(vec!["10.0.0.92:2379".to_string()], "root".to_string(), "CZlOeRySgF".to_string()).await?;

    client.get_client().delete("foo", None).await?;

    let keys = db::get_config_key_all(&mut client.get_client()).await?;

    for key in keys {
        println!("{}", key);
    }

    Ok(())
}
