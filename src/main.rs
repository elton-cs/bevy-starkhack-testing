use std::vec;

use starknet_crypto::poseidon_hash_many;
use starknet_ff::FieldElement;
use tokio_stream::StreamExt;
use torii_client::client::Client;

#[tokio::main]
async fn main() {
    println!("Running a rust torii client!");

    // client configuration
    let torii_url = "http://0.0.0.0:8080".to_string();
    let rpc_url = "http://0.0.0.0:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world: FieldElement = FieldElement::from_hex_be(
        "0xb4079627ebab1cd3cf9fd075dda1ad2454a7a448bf659591f259efa2519b18",
    )
    .unwrap();

    // create a new client
    let client: Client = Client::new(torii_url, rpc_url, relay_url, world, None)
        .await
        .unwrap();

    // create hash of all models' keys (in this case, just one: the player's contract addresss)
    let player_contract_address = FieldElement::from_hex_be(
        "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca",
    )
    .unwrap();
    let vec_keys = vec![player_contract_address.clone()];
    let hashed_keys = poseidon_hash_many(&vec_keys);

    // subscribe to the player's contract address
    let mut stream = client.on_entity_updated(vec![hashed_keys]).await.unwrap();

    // listen for incoming data
    loop {
        if let Some(data) = stream.next().await {
            println!("Incoming Data: {:?}", data);
            println!("----------");
        }
    }
}
