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
    let models_keys = None;

    let client = Client::new(torii_url, rpc_url, relay_url, world, models_keys)
        .await
        .unwrap();
}
