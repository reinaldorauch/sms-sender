table! {
    user (user_id) {
        user_id -> Int4,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        user_type -> Bpchar,
    }
}
