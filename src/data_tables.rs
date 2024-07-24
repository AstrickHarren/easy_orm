trait Relate {}

mod cake {
    use sqlx::PgExecutor;

    mod cols {
        pub struct Id(i32);
        pub struct Name(String);
    }
    pub use cols::*;

    struct Model {
        id: Id,
        name: Name,
    }

    enum Relation {
        Fruit,
    }

    impl Model {
        fn find_all<'c, E: PgExecutor<'c>>(e: E) {}

        fn find_related<'c, E: PgExecutor<'c>>(relation: Relation, e: E) {
            let query_1_m = format!(
                "SELECT {}.* FROM fruits where fruits.cake_id = $1",
                "fruits"
            );

            let query_m_m = format!(
                "SELECT {}.* FROM {} JOIN {} ON cake_fillings.{} = filling.id WHERE cake_filling.{} = $1",
                "fruit", "cake_fillings", "fillings", "filling_id", "cake_id"
                );
        }
    }
}
