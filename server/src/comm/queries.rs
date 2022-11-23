pub const CREATE_USER_TABLE_SQL: &'static str = r#"CREATE TABLE IF NOT EXISTS "users" (
    "username"	TEXT NOT NULL UNIQUE,
    "hash"	TEXT NOT NULL,
    PRIMARY KEY("username")
);
"#;