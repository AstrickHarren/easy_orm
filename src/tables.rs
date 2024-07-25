use crate::{
    relations::{Related, RelationBuilder, RelationDef, RelationTrait},
    sql::Col,
};
pub(super) mod cake {
    use super::*;

    pub struct Entity;
    pub struct Model {
        id: i32,
        name: String,
    }

    pub enum Column {
        Id,
        Name,
    }
    impl Into<Col> for Column {
        fn into(self) -> Col {
            match self {
                Column::Id => Col::new("cake".into(), "id".into()),
                Column::Name => Col::new("cake".into(), "name".into()),
            }
        }
    }
}

pub(super) mod fruit {
    use sqlx::{Postgres, QueryBuilder};

    use super::*;
    use crate::sql::Select;

    pub struct Entity;

    impl Entity {
        fn find() -> QueryBuilder<'static, Postgres> {
            Select::new("fruit".into()).query()
        }

        fn find_related<E>()
        where
            Self: Related<E>,
        {
        }
    }

    pub struct Model {
        id: i32,
        name: String,
        cake_id: i32,
    }

    pub enum Relation {
        Cake,
    }

    pub enum Column {
        Id,
        Name,
        CakeId,
    }
    impl Into<Col> for Column {
        fn into(self) -> Col {
            match self {
                Column::Id => Col::new("fruit".into(), "id".into()),
                Column::Name => Col::new("fruit".into(), "name".into()),
                Column::CakeId => Col::new("fruit".into(), "cake_id".into()),
            }
        }
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Cake => RelationBuilder::new()
                    .from(Column::Id)
                    .to(cake::Column::Id)
                    .into(),
            }
        }
    }

    impl Related<cake::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Cake.def()
        }
    }
}
