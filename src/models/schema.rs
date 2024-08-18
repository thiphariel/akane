// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(projects, users,);
