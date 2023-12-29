// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Int8,
        room_id -> Int8,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int8,
        leader_id -> Nullable<Int8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    players,
    rooms,
);
