use torii_grpc::types::schema::Entity;

pub fn get_current_position(entity: &Entity) -> (u32, u32) {
    let vec2 = entity.models[0].members[1].ty.as_struct().unwrap();

    let x = vec2.children[0]
        .ty
        .as_primitive()
        .unwrap()
        .as_u32()
        .unwrap();

    let y = vec2.children[1]
        .ty
        .as_primitive()
        .unwrap()
        .as_u32()
        .unwrap();

    (x, y)
}

pub fn get_current_moves(entity: &Entity) -> (u8, (u8, String)) {
    let remaining = entity.models[2].members[1]
        .ty
        .as_primitive()
        .unwrap()
        .as_u8()
        .unwrap();

    let last_direction_num = entity.models[2].members[2]
        .ty
        .as_enum()
        .unwrap()
        .option
        .unwrap();

    let last_direction_string = entity.models[2].members[2].ty.as_enum().unwrap().options
        [Into::<usize>::into(last_direction_num)]
    .name
    .clone();

    (remaining, (last_direction_num, last_direction_string))
}
