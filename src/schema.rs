// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        userid -> Int4,
        eventname -> Text,
        eventdescription -> Text,
        eventdate -> Date,
        eventdatetime -> Timestamp,
        #[max_length = 255]
        eventtype -> Varchar,
        eventcountry -> Text,
        eventcity -> Text,
        eventplace -> Text,
        eventimage -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

diesel::joinable!(events -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
