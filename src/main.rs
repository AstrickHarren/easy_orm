use easy_orm::{
    common::EntityTrait,
    data_table,
    relations::{Related, RelationTrait},
    sql::IntoCol,
};
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

data_table!(Cake of cakes {
    [id: i32],
    name: String,
    author: Option<String>,
});

data_table!(Fruits of fruits {
    [id: i32],
    name: String,
    cake_id: i32 => Cake.id
});

data_table!(Filling of fillings {
    [id: i32],
    name: String,
});

data_table!(CakeFilling of cake_fillings {
    cake_id: i32 => Cake.id,
    filling_id: i32 => Filling.id
});

impl Related<Cake> for Filling {
    fn to() -> easy_orm::relations::RelationDef {
        cake_filling::Relation::Cake.def()
    }

    fn via() -> Option<easy_orm::relations::RelationDef> {
        Some(cake_filling::Relation::Filling.def().rev())
    }
}

#[tokio::main]
async fn main() {
    let db = Db::new().await.unwrap();
    db.migrate().await.unwrap();

    // find all cakes with fillings id 2
    let cakes: Vec<cake::Row> = Filling::find_related::<Cake>()
        .filter(filling::Column::Id.eq(3))
        .all(&db.pool)
        .await
        .unwrap();
    dbg!(cakes);

    let fruits_of_chococake: Vec<fruits::Row> = Cake::find_related::<Fruits>()
        .filter(cake::Column::Id.eq(2))
        .all(&db.pool)
        .await
        .unwrap();
    dbg!(fruits_of_chococake);

    cake::Update::new(2)
        .name("ChocolateCake")
        .author("John".to_string())
        .query()
        .build()
        .execute(&db.pool)
        .await
        .unwrap();
    let cakes_with_grape: Vec<cake::Row> = Fruits::find_related::<Cake>()
        .filter(fruits::Column::Id.eq(2))
        .all(&db.pool)
        .await
        .unwrap();

    dbg!(cakes_with_grape);
}
