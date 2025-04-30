use sqlx::migrate::MigrateError;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use std::time::Duration;

/// 데이터베이스 설정 구조체
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub max_lifetime: Duration,
    pub idle_timeout: Duration,
    pub connect_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            connection_string: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in environment variables"),
            max_connections: 5,
            min_connections: 1,
            max_lifetime: Duration::from_secs(30 * 60), // 30 minutes
            idle_timeout: Duration::from_secs(10 * 60), // 10 minutes
            connect_timeout: Duration::from_secs(30),   // 30 seconds
        }
    }
}

impl DatabaseConfig {
    /// 새로운 데이터베이스 설정 인스턴스 생성
    pub fn new(
        connection_string: String,
        max_connections: u32,
        min_connections: u32,
        max_lifetime: Duration,
        idle_timeout: Duration,
        connect_timeout: Duration,
    ) -> Self {
        Self {
            connection_string,
            max_connections,
            min_connections,
            max_lifetime,
            idle_timeout,
            connect_timeout,
        }
    }

    /// 환경 변수에서 데이터베이스 설정 로드
    pub fn from_env() -> Self {
        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let max_lifetime = env::var("DATABASE_MAX_LIFETIME")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(30 * 60));

        let idle_timeout = env::var("DATABASE_IDLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(10 * 60));

        let connect_timeout = env::var("DATABASE_CONNECT_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(30));

        Self {
            connection_string: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in environment variables"),
            max_connections,
            min_connections,
            max_lifetime,
            idle_timeout,
            connect_timeout,
        }
    }

    /// 데이터베이스 연결 풀 생성
    pub async fn create_pool(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .max_lifetime(self.max_lifetime)
            .idle_timeout(self.idle_timeout)
            .acquire_timeout(self.connect_timeout)
            .connect(&self.connection_string)
            .await
    }
}

/// 데이터베이스 연결 풀 생성 헬퍼 함수
pub async fn create_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let config = DatabaseConfig::from_env();
    config.create_pool().await
}

/// 데이터베이스 마이그레이션 실행 함수
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
