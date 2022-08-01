table! {
    groups (id) {
        id -> Varchar,
        owner_id -> Varchar,
        name -> Varchar,
    }
}

table! {
    groups_users (id) {
        id -> Varchar,
        user_id -> Varchar,
        group_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(groups_users -> groups (group_id));
joinable!(groups_users -> users (user_id));

allow_tables_to_appear_in_same_query!(groups, groups_users, users,);
