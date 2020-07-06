table! {
    persons (id) {
        id -> Int4,
        name -> Varchar,
        birthdate -> Int8,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(persons -> users (user_id));

allow_tables_to_appear_in_same_query!(
    persons,
    users,
);
