use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "
            CREATE TABLE IF NOT EXISTS user_credentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS mappools (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS beatmap_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mappool_id INTEGER NOT NULL,
                beatmap_id INTEGER NOT NULL,
                artist TEXT NOT NULL,
                title TEXT NOT NULL,
                difficulty TEXT NOT NULL,
                mapper TEXT NOT NULL,
                mod_combination TEXT,
                category TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (mappool_id) REFERENCES mappools (id) ON DELETE CASCADE
            );
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_oauth_token_table",
            sql: "
            CREATE TABLE IF NOT EXISTS oauth_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                access_token TEXT NOT NULL,
                refresh_token TEXT NOT NULL,
                expires_in INTEGER NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "add_user_id_foreign_key_to_oauth_tokens",
            sql: "
            ALTER TABLE oauth_tokens ADD COLUMN user_id INTEGER REFERENCES user_credentials(id) ON DELETE CASCADE;
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "make_user_id_unique_in_oauth_tokens",
            sql: "
            CREATE UNIQUE INDEX IF NOT EXISTS idx_oauth_tokens_user_id ON oauth_tokens(user_id);
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 5,
            description: "recreate_oauth_tokens_with_username",
            sql: "
            DROP TABLE IF EXISTS oauth_tokens;
            
            CREATE TABLE oauth_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                irc_username TEXT NOT NULL UNIQUE,
                access_token TEXT NOT NULL,
                refresh_token TEXT NOT NULL,
                expires_in INTEGER NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        ",
            kind: MigrationKind::Up,
        },
    ]
}
