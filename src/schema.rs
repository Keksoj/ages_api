table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

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
        login_session -> Varchar,
    }
}

joinable!(login_history -> users (user_id));
joinable!(persons -> users (user_id));

allow_tables_to_appear_in_same_query!(login_history, persons, users,);
