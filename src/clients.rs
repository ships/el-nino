extern crate ridb_client;
extern crate ridb_client_extras;

use ridb_client::apis::*;

pub fn def_client() -> ridb_client::apis::client::APIClient {
    let api_key : String = match std::env::var("RIDB_API_KEY") {
        Ok(val) => val,
        Err(e) => panic!("no api key: {}", e),
    };

    let api_key_struct = configuration::ApiKey{
        prefix: None,
        key: api_key,
    };

    let mut config = configuration::Configuration::new();

    config.api_key = Some(api_key_struct);

    return client::APIClient::new(config);
}

pub fn ext_client() -> ridb_client_extras::apis::client::APIClient {
    let config = ridb_client_extras::apis::configuration::Configuration::new();
    return ridb_client_extras::apis::client::APIClient::new(config);
}
