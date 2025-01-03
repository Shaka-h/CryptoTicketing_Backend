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
        eventticketprice -> Int4,
    }
}

diesel::table! {
    likes (user_id) {
        user_id -> Int4,
        event_id -> Int4,
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
diesel::joinable!(likes -> events (event_id));
diesel::joinable!(likes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    likes,
    users,
);
