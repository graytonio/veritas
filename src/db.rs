use etcd_client::{Client, ConnectOptions, Error, GetOptions};

const CONFIG_KEY_PREFIX: &str = "config_key";

pub async fn get_config_key(client: &mut Client, index: i32) -> Result<Option<String>, Error> {
    let resp = client.get(format!("{}/{}", CONFIG_KEY_PREFIX, index), None).await?;

    if let Some(key) = resp.kvs().first() {
        return Ok(Some(key.value_str()?.to_string()));
    }

    return Ok(None);
}

pub async fn get_config_key_count(client: &mut Client) -> Result<i64, Error> {
    let resp = client.get(CONFIG_KEY_PREFIX, Some(GetOptions::new().with_prefix().with_count_only())).await?;
    Ok(resp.count())
}

pub async fn get_all_config_keys(client: &mut Client) -> Result<Vec<String>, Error> {
    let resp = client.get(CONFIG_KEY_PREFIX, Some(GetOptions::new().with_prefix())).await?;
    Ok(resp.kvs().iter().map(|ck| ck.value_str().unwrap().to_string()).collect::<Vec<String>>())
}

pub async fn add_config_key(client: &mut Client, key: String) -> Result<(), Error> {
    let next_index = get_config_key_count(client).await?;
    client.put(format!("{}/{}", CONFIG_KEY_PREFIX, next_index), key, None).await?;
    Ok(())
}

pub struct ConnectionManager {
    client: Client,
}

impl ConnectionManager {
    pub async fn new(endpoints: Vec<String>, username: String, password: String) -> Result<Self, Error> {
        let client = Client::connect(endpoints, Some(ConnectOptions::new().with_user(username, password))).await?;
        Ok(ConnectionManager {
            client
        })
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }
}
