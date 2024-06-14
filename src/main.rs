use std::vec;

use starknet_ff::FieldElement;
use torii_client::client::Client;
use torii_grpc::types::KeysClause;

#[tokio::main]
async fn main() {
    println!("Running a rust torii client!");

    let torii_url = "http://0.0.0.0:8080".to_string();
    let rpc_url = "http://0.0.0.0:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world: FieldElement = FieldElement::from_hex_be(
        "0xb4079627ebab1cd3cf9fd075dda1ad2454a7a448bf659591f259efa2519b18",
    )
    .unwrap();

    let client = Client::new(torii_url, rpc_url, relay_url, world, None)
        .await
        .unwrap();

    let position_key = FieldElement::from_hex_be(
        "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca",
    )
    .unwrap();
    let vec_keys = vec![position_key];

    let position_key_clause = KeysClause {
        model: "Position".to_string(),
        keys: vec_keys,
    };

    if let Some(data) = client.model(&position_key_clause).await.unwrap() {
        println!("POSITION MODEL: {:?}", data);
    }
}
