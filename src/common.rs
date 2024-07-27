use std::{marker::PhantomData, sync::Arc};

use sqlx::{postgres::PgRow, FromRow, Postgres, Row};

use crate::{
    relations::Related,
    sql::{Col, Iden, Select},
};

pub trait EntityTrait {
    const TABLE_NAME: &'static str;
    type Column;
    type Row;

    fn all_col() -> Col {
        Col::new(Self::TABLE_NAME.into(), "*".into())
    }

    fn find() -> Select<Self>
    where
        Self: Sized,
    {
        Select::new(Self::TABLE_NAME.into())
    }

    fn find_related<E>() -> Select<E>
    where
        Self: Related<E>,
        E: EntityTrait,
    {
        let mut sql = Select::new(E::TABLE_NAME.into());
        if let Some(via) = Self::via() {
            sql = sql.join(via)
        }
        sql.join(Self::to())
    }
}

pub trait ColumnTrait {
    const TABLE_NAME: &'static str;
    const COL_NAME: &'static str;

    type Data: for<'r> FromRow<'r, PgRow>;

    // TODO: stop copying (using Cow?)
    fn col() -> Col {
        Col {
            tbl: Self::TABLE_NAME.into(),
            col: Self::COL_NAME.into(),
        }
    }
}

pub trait ColumnList {
    type Extractor: for<'r> FromRow<'r, PgRow>;
    type Extracted: From<Self::Extractor>;
    fn cols() -> impl Iterator<Item = Col>;
}
