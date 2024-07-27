use std::i64;

use easy_orm::{
    common::EntityTrait,
    data_table, many_to_many,
    relations::{Related, RelationTrait},
    sql::IntoCol,
};
use sqlx::{migrate::MigrateError, postgres::PgConnectOptions, PgPool};

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

data_table!(Circle of circles {
    [id: i64],
    name: String,
    is_connected: bool,
    super_circle_id: Option<i64>,
    admin_circle_id: Option<i64>,
});

data_table!(Person of people {
    [ id: i64 ],
    first_name: String,
    last_name: Option<String>,
    external_identity_provider: Option<String>,
    external_identity_number: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    city: Option<String>,
    state_or_province: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
});

data_table!(Organization of organizations {
    [id: i64],
    name: String,
    circle_id: i64 => Circle.id,
    website: Option<String>,
    external_identity_provider: Option<String>,
    external_identity_number: Option<String>,
});

data_table!(Service of services {
    [id: i64],
    name: String,
    organization_id: i64 => Organization.id,
});

data_table!(PeopleInCircle of people_in_circle {
    [id: i64],
    circle_id: i64 => Circle.id,
    person_id: i64 => Person.id,
});

data_table!(ServicesInCircle of services_in_circle {
    [id: i64],
    circle_id: i64 => Circle.id,
    service_id: i64 => Service.id,
});

many_to_many!(person - people_in_circle - circle);
many_to_many!(service - services_in_circle - circle);
many_to_many!(cake - cake_filling - filling);

#[tokio::main]
async fn main() {
    let db = Db::new().await.unwrap();
    db.migrate().await.unwrap();

    let (rnd_id, rnd_name, rnd) = Circle::find()
        .col((Circle::Id, Circle::Name, Circle))
        .filter(Circle::Name.eq("RND"))
        .one(&db.pool)
        .await
        .unwrap();

    let rnd_people = Circle::find_related::<Person>()
        .filter(Circle::Id.eq(rnd_id))
        .all(&db.pool)
        .await
        .unwrap();

    let itai = InsertPerson {
        first_name: "Itai".to_string(),
        ..Default::default()
    }
    .insert(&db.pool)
    .await
    .unwrap();

    dbg!(rnd_id, rnd_name, rnd, rnd_people, itai);
}
