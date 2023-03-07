use etcd_client::{Client, ConnectOptions, Error};

const CONFIG_KEY_ARRAY_KEY: &str = "valid_config_keys";

// TODO Convert from Vec<String> to HashSet<String> for storing config keys

pub async fn get_config_key_all(client: &mut Client) -> Result<Vec<String>, Error> {
    let resp = client.get(CONFIG_KEY_ARRAY_KEY, None).await?;

    match resp.kvs().first() {
        Some(key) => Ok(key.value_str()?.split(";").map(|s| s.to_string()).collect::<Vec<String>>()),
        None => Ok(Vec::new())
    }
}

pub async fn get_config_key(client: &mut Client, index: usize) -> Result<Option<String>, Error> {
    let keys = get_config_key_all(client).await?;
    Ok(keys.get(index).cloned())
}

pub async fn get_config_key_count(client: &mut Client) -> Result<usize, Error> {
    Ok(get_config_key_all(client).await?.len())
}

async fn update_config_keys(client: &mut Client, keys: Vec<String>) -> Result<(), Error> {
    client.put(CONFIG_KEY_ARRAY_KEY, keys.join(";"), None).await?;
    Ok(())
}

pub async fn add_config_key(client: &mut Client, key: String) -> Result<(), Error> {
    let mut keys = get_config_key_all(client).await?;
    keys.push(key);
    update_config_keys(client, keys).await?;
    Ok(())
}

pub async fn remove_config_key(client: &mut Client, key: String) -> Result<(), Error> { 
    let keys = get_config_key_all(client).await?;
    let new_keys = keys.into_iter().filter(|k| *k != key).collect::<Vec<String>>();
    update_config_keys(client, new_keys).await?;
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
