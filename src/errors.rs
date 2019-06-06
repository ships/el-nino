extern crate ridb_client;
extern crate ridb_client_extras;

pub fn error_str(e: ridb_client::apis::Error) -> String {
    return match e {
        ridb_client::apis::Error::Reqwest(e) => format!("uri error: {}", e),
        ridb_client::apis::Error::Serde(e) => format!("encoding error: {}", e),
    }
}

pub fn error_str_xt(e: ridb_client_extras::apis::Error) -> String {
    return match e {
        ridb_client_extras::apis::Error::Reqwest(e) => format!("uri error: {}", e),
        ridb_client_extras::apis::Error::Serde(e) => format!("encoding error: {}", e),
    }
}
