use std::{marker::PhantomData, sync::Arc};

use sqlx::{postgres::PgRow, FromRow, Postgres, Row};

use crate::{
    relations::Related,
    sql::{Col, Iden, Select},
};

pub trait EntityTrait {
    const TABLE_NAME: &'static str;
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

pub trait Selector {
    type Data;
    fn cols() -> impl Iterator<Item = Col>;
    fn from_row(row: &PgRow) -> Result<Self::Data, sqlx::Error>;
}

impl<A, B> Selector for (A, B)
where
    A: Selector,
    B: Selector,
{
    type Data = (A::Data, B::Data);
    fn cols() -> impl Iterator<Item = Col> {
        A::cols().chain(B::cols())
    }
    fn from_row(row: &PgRow) -> Result<Self::Data, sqlx::Error> {
        Ok((A::from_row(row)?, B::from_row(row)?))
    }
}

pub trait ColumnList {
    type Extractor: for<'r> FromRow<'r, PgRow>;
    type Extracted: From<Self::Extractor>;
    fn cols() -> impl Iterator<Item = Col>;
}
