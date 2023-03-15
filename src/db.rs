use std::collections::{HashSet, HashMap};
use log::info;
use etcd_client::{Client, ConnectOptions, Error, GetOptions};

const CONFIG_KEY_ARRAY_KEY: &str = "valid_config_keys";

pub type ConfigSchema = HashSet<String>;
pub type NodeConfig = HashMap<String, String>;

pub async fn get_config_key_all(client: &mut Client) -> Result<ConfigSchema, Error> {
    let resp = client.get(CONFIG_KEY_ARRAY_KEY, None).await?;

    match resp.kvs().first() {
        Some(key) => Ok(key.value_str()?.split(";").map(|s| s.to_string()).collect::<ConfigSchema>()),
        None => Ok(HashSet::new())
    }
}

pub async fn is_valid_config_key(client: &mut Client, key: String) -> Result<bool, Error> {
    let keys = get_config_key_all(client).await?;
    Ok(keys.contains(&key))
}

pub async fn get_config_key_count(client: &mut Client) -> Result<usize, Error> {
    Ok(get_config_key_all(client).await?.len())
}

pub async fn update_config_keys(client: &mut Client, keys: ConfigSchema) -> Result<(), Error> {
    client.put(CONFIG_KEY_ARRAY_KEY, keys.into_iter().collect::<Vec<String>>().join(";"), None).await?;
    Ok(())
}

pub async fn add_config_key(client: &mut Client, key: String) -> Result<bool, Error> {
    let mut keys = get_config_key_all(client).await?;
    let added = keys.insert(key.clone());
    info!(target: "config_key_manager", "Adding key {} to configuration schema: success={}", &key, added);
    update_config_keys(client, keys).await?;
    Ok(added)
}


pub async fn remove_config_key(client: &mut Client, key: String) -> Result<bool, Error> { 
    let mut keys = get_config_key_all(client).await?;
    let removed = keys.remove(&key);
    update_config_keys(client, keys).await?;
    Ok(removed)
}

pub async fn get_node_config_value_all(client: &mut Client, node_name: String) -> Result<NodeConfig, Error> {
    let keys = client.get(node_name, Some(GetOptions::new().with_prefix())).await?;

    let mut node_conf = NodeConfig::new();

    for k in keys.kvs().iter() {
        let key = k.key_str()?.to_string();
        let value = k.value_str()?.to_string();
        node_conf.insert(key, value);
    }
    Ok(node_conf.to_owned())
}

pub async fn get_node_config_value(client: &mut Client, node_name: String, key: String) -> Result<String, Error> {
    let valid = is_valid_config_key(client, key.clone()).await?;
    if !valid {
        return Err(Error::InvalidArgs("Invalid Config Key".to_string()))
    }

    let resp = client.get(format!("{}/{}", node_name, key), None).await?;

    match resp.kvs().first() {
        Some(val) => Ok(val.value_str()?.to_string()),
        None => Ok(String::new()), // TODO Check groups for key values
    }
}

pub async fn set_node_config_value(client: &mut Client, node_name: String, key: String, value: String) -> Result<(), Error> {
    let valid = is_valid_config_key(client, key.clone()).await?;
    if !valid {
        return Err(Error::InvalidArgs("Invalid Config Key".to_string()))
    }

    client.put(format!("{}/{}", node_name, key), value, None).await?;
    Ok(())
}

#[derive(Clone)]
pub struct ConnectionManager {
    client: Client,
}

impl ConnectionManager {
    pub async fn new(endpoints: Vec<String>, auth: Option<EtcdAuth>) -> Result<Self, Error> {
        
        let client = match auth {
            Some(a) => Client::connect(endpoints, Some(ConnectOptions::new().with_user(a.username, a.password))).await?,
            None => Client::connect(endpoints, None).await?,
        };

        Ok(ConnectionManager {
            client
        })
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }
}

pub struct EtcdAuth {
    username: String,
    password: String
}

impl EtcdAuth {
    pub fn new(username: String, password: String) -> Self {
        EtcdAuth { username, password }
    }
}

