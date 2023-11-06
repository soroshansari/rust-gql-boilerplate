// @generated automatically by Diesel CLI.

diesel::table! {
    thermostat_status (id) {
        id -> Int4,
        status -> Bool,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 120]
        email -> Varchar,
        password_hash -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(thermostat_status, users,);
