use crate::{
    common::EntityTrait,
    relations::{Related, RelationBuilder, RelationDef, RelationTrait},
    sql::{Col, Select},
};

pub(super) mod cake {
    use super::*;
    use crate::common::EntityTrait;

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

    impl Related<filling::Entity> for Entity {
        fn to() -> RelationDef {
            cake_filling::Relation::Cake.def().rev()
        }

        fn via() -> Option<RelationDef> {
            cake_filling::Relation::Filling.def().into()
        }
    }

    impl Related<fruit::Entity> for Entity {
        fn to() -> RelationDef {
            fruit::Relation::Cake.def().rev()
        }
    }

    impl Entity {
        pub fn find_related<E>() -> Select
        where
            Self: Related<E>,
            E: EntityTrait,
        {
            let mut sql = Select::new(E::TABLE_NAME.into()).col(E::all_col());
            if let Some(via) = Self::via() {
                sql = sql.join(via)
            }
            sql.join(Self::to())
        }
    }
}
pub(super) mod filling {
    use super::*;

    pub struct Entity;

    impl EntityTrait for Entity {
        type Column = Column;

        const TABLE_NAME: &'static str = "filling";
    }
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
                Column::Id => Col::new("filling".into(), "id".into()),
                Column::Name => Col::new("filling".into(), "name".into()),
            }
        }
    }
}

enum A {}
impl A {
    fn s(&self) -> () {
        match self {
            _ => (),
        }
    }
}

pub(super) mod cake_filling {
    use super::*;

    pub struct Entity;
    pub enum Column {
        CakeId,
        FillingId,
    }

    impl Into<Col> for Column {
        fn into(self) -> Col {
            match self {
                Column::CakeId => Col::new("cake_filling".into(), "cake_id".into()),
                Column::FillingId => Col::new("cake_filling".into(), "filling_id".into()),
            }
        }
    }

    pub enum Relation {
        Cake,
        Filling,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> crate::relations::RelationDef {
            match self {
                Relation::Cake => RelationBuilder::new()
                    .from(Column::CakeId)
                    .to(cake::Column::Id)
                    .into(),
                Relation::Filling => RelationBuilder::new()
                    .from(Column::FillingId)
                    .to(filling::Column::Id)
                    .into(),
            }
        }
    }
}

pub(super) mod fruit {
    use super::*;

    pub struct Entity;

    impl EntityTrait for Entity {
        type Column = Column;

        const TABLE_NAME: &'static str = "fruit";
    }

    impl Entity {
        pub fn find() -> Select<()> {
            Select::new("fruit".into())
        }

        pub fn find_related<E>() -> Select<()>
        where
            Self: Related<E>,
        {
            let to = Self::to();
            let tbl = to.to_col.tbl.clone();

            Select::new(tbl.clone())
                .col(Col::new(tbl, "*".into()))
                .join(to)
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
                    .from(Column::CakeId)
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

#[cfg(test)]
mod test {
    use super::{cake, fruit};
    use crate::tables::{filling, A};

    #[test]
    fn test() {
        let sql = cake::Entity::find_related::<fruit::Entity>();
        let sql = sql.query().into_sql();
        println!("{}", sql);

        let sql = cake::Entity::find_related::<filling::Entity>();
        println!("{}", sql.query().into_sql());
    }
}
