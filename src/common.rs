use crate::{
    relations::Related,
    sql::{Col, Select},
};

pub trait EntityTrait {
    const TABLE_NAME: &'static str;
    type Column;

    fn all_col() -> Col {
        Col::new(Self::TABLE_NAME.into(), "*".into())
    }

    fn find() -> Select {
        Select::new(Self::TABLE_NAME.into()).col(Self::all_col())
    }

    fn find_related<E>() -> Select
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
