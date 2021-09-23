use ipfs_api::{IpfsApi, IpfsClient};
use std::fs::File;

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::default();
    let file = File::open("Cargo.toml").expect("could not read source file");
    let mut cid: String = "".to_string();
    
    match client.add(file).await {
        Ok(res) => cid = res.hash.to_string(),
        Err(e) => eprintln!("error adding file: {}", e),
    }
    println!("{}", cid);
}
