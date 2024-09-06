// @generated automatically by Diesel CLI.

diesel::table! {
    auth_password (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        salt -> Text,
        password -> Text,
        update_time -> Integer,
    }
}

diesel::table! {
    auth_session (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        session_key -> Text,
        creation_time -> Integer,
        last_access_time -> Integer,
        expiry_time -> Integer,
        max_inactive_interval -> Integer,
    }
}

diesel::table! {
    auth_token (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        token -> Text,
        count -> Integer,
        quota -> Integer,
        quota_type -> Integer,
        expiry_time -> Integer,
    }
}

diesel::table! {
    filter_group (id) {
        id -> Nullable<Integer>,
        name -> Text,
        desc -> Text,
        priority -> Integer,
        from_lang -> Text,
        to_lang -> Text,
        ignore -> Bool,
    }
}

diesel::table! {
    filter_line (id) {
        id -> Nullable<Integer>,
        group_id -> Integer,
        degree -> Integer,
        use_regex -> Bool,
        from_sentence -> Text,
        to_sentence -> Text,
        include -> Text,
        exclude -> Text,
        ignore -> Bool,
    }
}

diesel::table! {
    log_activity (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        title -> Text,
        desc -> Text,
        time -> Integer,
    }
}

diesel::table! {
    log_api (id) {
        id -> Nullable<Integer>,
        token_id -> Integer,
        status_code -> Integer,
        endpoint -> Text,
        time -> Integer,
    }
}

diesel::table! {
    log_auth (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        status_code -> Integer,
        device_id -> Text,
        ip_addr -> Text,
        fail_count -> Integer,
        fail_reason -> Text,
        time -> Integer,
    }
}

diesel::table! {
    user_account (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    user_permission (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        perm_name -> Text,
        perm_action -> Text,
    }
}

diesel::table! {
    user_profile (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
        desc -> Text,
        image -> Binary,
        join_time -> Integer,
        update_time -> Integer,
    }
}

diesel::joinable!(auth_password -> user_account (user_id));
diesel::joinable!(auth_session -> user_account (user_id));
diesel::joinable!(auth_token -> user_account (user_id));
diesel::joinable!(filter_line -> filter_group (group_id));
diesel::joinable!(log_activity -> user_account (user_id));
diesel::joinable!(log_api -> auth_token (token_id));
diesel::joinable!(log_auth -> user_account (user_id));
diesel::joinable!(user_permission -> user_account (user_id));
diesel::joinable!(user_profile -> user_account (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_password,
    auth_session,
    auth_token,
    filter_group,
    filter_line,
    log_activity,
    log_api,
    log_auth,
    user_account,
    user_permission,
    user_profile,
);
