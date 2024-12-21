// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Nullable<Integer>,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        address -> Text,
        phone -> Text,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
    }
}
