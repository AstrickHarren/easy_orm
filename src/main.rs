use sqlx::{
    migrate::MigrateError,
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

struct Db {
    pool: PgPool,
}

impl Db {
    async fn new() -> Result<Self, sqlx::Error> {
        let url = "postgres://localhost:5432";
        let opts: PgConnectOptions = url.parse()?;
        let opts = opts.username("postgres").password("postgres");
        let pool = PgPool::connect_with(opts).await?;
        Ok(Self { pool })
    }

    async fn migrate(&self) -> Result<(), MigrateError> {
        sqlx::migrate!("./migration").run(&self.pool).await
    }
}

#[tokio::main]
async fn main() {
    let db = Db::new().await.unwrap();
    db.migrate().await.unwrap();
}
