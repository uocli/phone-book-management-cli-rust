// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Nullable<Integer>,
        first_name -> Text,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Text,
        address -> Nullable<Text>,
        created_date -> Nullable<Text>,
    }
}
