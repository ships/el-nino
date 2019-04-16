extern crate ridb_client;
extern crate futures;

use ridb_client::apis::*;

fn main() {
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

    let c = client::APIClient::new(config);
    let p = c.permit_entrances_api();

    let permit_entr_id = "376";
    let res = p.get_permit_entrance(&permit_entr_id);

    match res {
      Ok(ent) => println!("Got body: "),
      Err(e) => println!("Got error: {}", error_str(e)),
    }

    println!("Done!");
}

fn error_str(e: ridb_client::apis::Error) -> String {
    return match e {
        ridb_client::apis::Error::Reqwest(e) => format!("uri error: {}", e),
        ridb_client::apis::Error::Serde(e) => format!("encoding error: {}", e),
    }
}
