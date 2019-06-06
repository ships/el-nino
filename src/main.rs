extern crate futures;

mod clients;
mod errors;

use errors::*;

fn main() {
    let c = clients::def_client();
    let p = c.permit_entrances_api();

    let res = p.get_permit_entrances(0,0);

    match res {
      Ok(ent) => println!("Got ids: {}", body_str(ent)),
      Err(e) => println!("Got error: {}", error_str(e)),
    }

    println!("Done!");
}

fn body_str(r: ridb_client::models::InlineResponse2006) -> String {
    match r.RECDATA() {
        None => return String::from("none!"),
        Some(v) => {
            let vec: Vec<_> = v.into_iter().map(|x| format!("id: {}, desc: {}\n", x.facility_id(),x.permit_entrance_name())).collect();
            return format!("{:?}", vec)
        }
    }
}

struct Relay{}

impl Relay {
}
