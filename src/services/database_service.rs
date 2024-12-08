use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;
use std::fs;

#[derive(Clone)] 
pub struct DatabaseService {
    pub pool: Pool<Sqlite>,
}

impl DatabaseService {
    pub async fn new() -> Self {
        let current_dir = env::current_dir().expect("Falha ao obter o diretório de trabalho atual.");
        let database_dir = current_dir.join("data");
        let database_path = database_dir.join("assistant_mama_backend.db");

        if !database_dir.exists() {
            fs::create_dir_all(&database_dir).expect("[ERROR] Falha ao criar o diretório 'data'");
        }

        if !database_path.exists() {
            fs::File::create(&database_path).expect("[ERROR] Falha ao criar o arquivo de banco de dados");
        }

        let absolute_path = database_path.canonicalize().expect("Falha ao obter o caminho absoluto do banco de dados.");
        let database_url = format!("sqlite:///{}", absolute_path.display());

        let pool = SqlitePoolOptions::new()
            .connect(&database_url)
            .await
            .expect("[ERROR] Falha ao conectar ao banco de dados SQLite");

        DatabaseService { pool }
    }

    pub async fn initialize_database(&self) {
        let query = r#"
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                datetime TEXT NOT NULL,
                event_type TEXT NOT NULL,
                command TEXT,
                user_text TEXT,
                system_response TEXT,
                emotion TEXT,
                duration REAL,
                status TEXT
            );
        "#;

        sqlx::query(query).execute(&self.pool).await.expect("[ERROR] Falha ao criar a tabela de eventos");
    }

    pub async fn insert_event(
        &self,
        event_type: &str,
        command: Option<&str>,
        user_text: Option<&str>,
        system_response: Option<&str>,
        emotion: Option<&str>,
        duration: Option<f64>,
        status: Option<&str>,
    ) {
        let query = r#"
            INSERT INTO events (datetime, event_type, command, user_text, system_response, emotion, duration, status)
            VALUES (datetime('now'), ?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(query)
            .bind(event_type)
            .bind(command)
            .bind(user_text)
            .bind(system_response)
            .bind(emotion)
            .bind(duration)
            .bind(status)
            .execute(&self.pool)
            .await
            .expect("[ERROR] Falha ao inserir evento no banco de dados");
    }

    pub async fn close(&self) {
        println!("[INFO] Fechando a conexão com o banco de dados...");
        self.pool.close().await;
        println!("[SUCCESS] Conexão com o banco de dados fechada com sucesso.");
    }
}
