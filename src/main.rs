mod db;
use etcd_client::{Error};

const KEYS: [&str; 4] = ["foo", "bar", "region", "access_db_hostname"];

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = db::ConnectionManager::new(vec!["10.0.0.92:2379".to_string()], "root".to_string(), "CZlOeRySgF".to_string()).await?;

    let keys = db::get_all_config_keys(&mut client.get_client()).await?;

    for key in keys {
        println!("{}", key);
    }

    Ok(())
}
