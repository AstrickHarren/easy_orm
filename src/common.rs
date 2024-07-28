
use sqlx::{postgres::PgRow, FromRow};

use crate::{
    relations::Related,
    sql::{Col, JoinTy, Select},
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
            sql = sql.join(JoinTy::Left, via)
        }
        sql.join(JoinTy::Left, Self::to())
    }
}

pub trait Selector {
    type Data;
    fn cols() -> impl Iterator<Item = Col>;
    fn from_row(row: &PgRow) -> Result<Self::Data, sqlx::Error>;
}

impl Selector for () {
    type Data = ();
    fn cols() -> impl Iterator<Item = Col> {
        std::iter::empty()
    }
    fn from_row(_: &PgRow) -> Result<Self::Data, sqlx::Error> {
        Ok(())
    }
}

// implementing `Selector` for tuples
macro_rules! impl_selector {
    ($($t:ident),*) => {
        impl<$($t),*> Selector for ($($t),*)
            where $($t: Selector),*
        {
            type Data = ($($t::Data),*);
            fn cols() -> impl Iterator<Item = Col> {
                std::iter::empty()$(.chain($t::cols()))*
            }

            fn from_row(row: &PgRow) -> Result<Self::Data, sqlx::Error> {
                Ok(($($t::from_row(row)?),*))
            }
        }
    };
}

macro_rules! impl_selector_for_tuples {
    ($head:ident, $tail:ident) => {
        impl_selector!($head, $tail);
    };
    ($head:ident, $($tail:ident),*) => {
        impl_selector_for_tuples!($($tail),*);
        impl_selector!($head, $($tail),*);
    };
}

impl_selector_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);

pub trait ColumnList {
    type Extractor: for<'r> FromRow<'r, PgRow>;
    type Extracted: From<Self::Extractor>;
    fn cols() -> impl Iterator<Item = Col>;
}
