// @generated automatically by Diesel CLI.

diesel::table! {
    appuser (id) {
        id -> Int4,
        hpassword -> Varchar,
        username -> Varchar,
        firstname -> Nullable<Varchar>,
        patronym -> Nullable<Varchar>,
        surname -> Nullable<Varchar>,
        rt -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_change (id) {
        id -> Int4,
        created -> Float8,
        action -> Varchar,
        user_id -> Int4,
    }
}

diesel::joinable!(user_change -> appuser (user_id));

diesel::allow_tables_to_appear_in_same_query!(appuser, user_change,);
