use torii_grpc::types::schema::Entity;

pub fn get_current_position(entity: &Entity) {
    let position = entity
        .models
        .iter()
        .find(|m| m.name == "Position".to_string());

    if let Some(model) = position {
        println!("Model name: {:?}", model.name);
        let item = model
            .members
            .iter()
            .find(|x| x.name == "vec".to_string())
            .unwrap()
            .ty
            .as_struct()
            .unwrap();

        let x = item
            .children
            .first()
            .unwrap()
            .ty
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        let y = item
            .children
            .last()
            .unwrap()
            .ty
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        println!("x: {:?}, y: {:?}", x, y);
    } else {
        println!("No Position model found.");
    }
}

pub fn get_current_moves() {}
