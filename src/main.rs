use std::vec;

use starknet_crypto::poseidon_hash_many;
use starknet_ff::FieldElement;
use tokio_stream::StreamExt;
use torii_client::client::Client;
use torii_grpc::types::{schema::Entity, Clause, KeysClause, Query};

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

    let client: Client = Client::new(torii_url, rpc_url, relay_url, world, None)
        .await
        .unwrap();

    // example 1: manually calling the torii server to get a model's data
    // let position_key_clause = build_position_key_clause();
    // if let Some(data) = client.model(&position_key_clause).await.unwrap() {
    //     println!("POSITION MODEL: {:?}", data);
    // }

    // example 2: Querying the torii server for a list of entities
    // let query = Query {
    //     // clause: None,
    //     clause: Some(Clause::Keys(build_position_key_clause())),
    //     limit: 100,
    //     offset: 0,
    // };
    // let entities = client.entities(query).await.unwrap();
    // for entity in entities {
    //     println!("ENTITY: {:?}", entity);
    // }

    // let subscription = client.start_subscription().await.unwrap();
    // subscription.await;

    let player_contract_address = FieldElement::from_hex_be(
        "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca",
    )
    .unwrap();
    let vec_keys = vec![player_contract_address.clone()];

    let hashed_keys = poseidon_hash_many(&vec_keys);
    let mut stream = client.on_entity_updated(vec![hashed_keys]).await.unwrap();

    loop {
        if let Some(data) = stream.next().await {
            println!("Incoming Data: {:?}", data);
            println!("----------");
        }
    }
}

fn build_position_key_clause() -> KeysClause {
    let position_key = FieldElement::from_hex_be(
        "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca",
    )
    .unwrap();
    let vec_keys = vec![position_key];

    let position_key_clause = KeysClause {
        model: "Moves".to_string(),
        keys: vec_keys,
    };

    position_key_clause
}
