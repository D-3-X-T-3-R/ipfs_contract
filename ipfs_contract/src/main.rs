use ipfs_api::{IpfsApi, IpfsClient};
use std::fs::File;
use crate::contract::storage::Storage;

mod configuration_parameters;
mod contract;

#[actix_rt::main]
async fn main() {
    let app_name = "ipfs_contract";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    let client = IpfsClient::default();
    let file = File::open(config_param.file_path()).expect(&format!(
        "could not read source file at {}",
        config_param.file_path()
    ));
    
    let mut cid: String = "".to_string();

    match client.add(file).await {
        Ok(res) => cid = res.hash.to_string(), // generating cid
        Err(e) => eprintln!("error adding file: {}", e),
    }
    let stored_cid=Storage::new(cid.to_string()); //storing cid
    println!("{}",stored_cid.get()); // printing stored cid
}
