// @generated automatically by Diesel CLI.

diesel::table! {
    users (id, username) {
        id -> Uuid,
        username -> Text,
        hash -> Text,
    }
}
