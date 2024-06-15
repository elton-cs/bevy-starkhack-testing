use bevy_starkhack_testing::handling::get_current_position;
use starknet_crypto::poseidon_hash_many;
use starknet_ff::FieldElement;
use std::vec;
use tokio_stream::StreamExt;
use torii_client::client::Client;
use torii_grpc::client::EntityUpdateStreaming;

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

    // let metadata = client.metadata();
    // let schema = metadata.model("Position").unwrap().schema.clone();
    // println!("Metadata: {:?}", schema);

    let mut stream = get_entities_stream(&client).await;

    while let Some(data) = stream.next().await {
        match data {
            Ok(data) => {
                if data.models.is_empty() {
                    println!("Skipping initialization response.");
                } else {
                    get_current_position(&data);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

async fn get_entities_stream(client: &Client) -> EntityUpdateStreaming {
    // create hash of all models' keys (in this case, just one: the player's contract addresss)
    let player_contract_address = FieldElement::from_hex_be(
        "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca",
    )
    .unwrap();
    let vec_keys = vec![player_contract_address.clone()];
    let hashed_keys = poseidon_hash_many(&vec_keys);

    // subscribe to the player's contract address
    let stream = client.on_entity_updated(vec![hashed_keys]).await.unwrap();

    stream
}

// todo: find what the event_key represents
async fn _get_events_stream(client: &Client) -> EntityUpdateStreaming {
    // create hash of all models' keys (in this case, just one: the player's contract addresss)
    let event_key = FieldElement::from_hex_be(
        "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca",
    )
    .unwrap();
    let vec_keys = vec![event_key.clone()];
    let hashed_keys = poseidon_hash_many(&vec_keys);

    // subscribe to the player's contract address
    let stream = client
        .on_event_message_updated(vec![hashed_keys])
        .await
        .unwrap();

    stream
}
