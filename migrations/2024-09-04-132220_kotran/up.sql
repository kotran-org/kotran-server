PRAGMA foreign_keys = 1;

CREATE TABLE auth_password (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    salt TEXT NOT NULL,
    password TEXT NOT NULL,
    update_time INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE
);

CREATE TABLE auth_session (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    session_key TEXT NOT NULL UNIQUE,
    creation_time INTEGER NOT NULL,
    last_access_time INTEGER NOT NULL,
    expiry_time INTEGER NOT NULL,
    max_inactive_interval INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE
);

CREATE TABLE auth_token (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    count INTEGER NOT NULL,
    quota INTEGER NOT NULL,
    quota_type INTEGER NOT NULL,
    expiry_time INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE
);

CREATE TABLE filter_group (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    desc TEXT NOT NULL,
    priority INTEGER NOT NULL,
    from_lang TEXT NOT NULL,
    to_lang TEXT NOT NULL,
    ignore BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE filter_line (
    id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL,
    degree INTEGER NOT NULL,
    use_regex BOOLEAN NOT NULL DEFAULT 0,
    from_sentence TEXT NOT NULL,
    to_sentence TEXT NOT NULL,
    include TEXT NOT NULL,
    exclude TEXT NOT NULL,
    ignore BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (group_id) REFERENCES filter_group(id) ON DELETE CASCADE
);

CREATE TABLE log_auth (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    status_code INTEGER NOT NULL,
    device_id TEXT NOT NULL,
    ip_addr TEXT NOT NULL,
    fail_count INTEGER NOT NULL,
    fail_reason TEXT NOT NULL,
    time INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE NO ACTION
);

CREATE TABLE log_api (
    id INTEGER PRIMARY KEY,
    token_id INTEGER NOT NULL,
    status_code INTEGER NOT NULL,
    endpoint TEXT NOT NULL,
    time INTEGER NOT NULL,
    FOREIGN KEY (token_id) REFERENCES auth_token(id) ON DELETE NO ACTION
);

CREATE TABLE log_activity (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    desc TEXT NOT NULL,
    time INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE NO ACTION
);

CREATE TABLE user_account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE user_profile (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    desc TEXT NOT NULL,
    image BLOB NOT NULL,
    join_time INTEGER NOT NULL,
    update_time INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE
);

CREATE TABLE user_permission (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    perm_name TEXT NOT NULL,
    perm_type INTEGER NOT NULL,
    perm_level INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id) ON DELETE CASCADE
);