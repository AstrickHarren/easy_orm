use std::{fmt::Display, marker::PhantomData, sync::Arc};

use futures::{StreamExt, TryStreamExt};
use itertools::Itertools;
use sqlx::{Encode, FromRow, PgExecutor, Postgres, QueryBuilder, Type};

use crate::{
    common::{ColumnList, Selector},
    relations::RelationDef,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Iden {
    iden: Arc<String>,
}

impl Display for Iden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iden.fmt(f)
    }
}

impl From<&str> for Iden {
    fn from(value: &str) -> Self {
        Self {
            iden: Arc::new(value.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Col {
    pub tbl: Iden,
    pub col: Iden,
}

impl Col {
    pub fn new(tbl: Iden, col: Iden) -> Self {
        Self { tbl, col }
    }
}

impl Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.tbl, self.col)
    }
}

#[derive(Default, Debug)]
pub enum JoinTy {
    #[default]
    Inner,
    Outer,
    Left,
    Right,
}

impl Display for JoinTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

pub(crate) struct Join {
    ty: JoinTy,
    tbl: Iden,
    from_col: Col,
    to_col: Col,
}

impl Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} JOIN {} ON {} = {}",
            self.ty, self.tbl, self.from_col, self.to_col
        )
    }
}

pub trait Filter<'q>: Sized {
    fn filter(self, builder: &mut QueryBuilder<'q, Postgres>);
    fn effective(&self) -> bool {
        true
    }
}

pub trait IntoCol: Into<Col> {
    fn eq<T>(self, val: T) -> ColEq<T> {
        ColEq {
            col: self.into(),
            val,
        }
    }

    fn is_null(self, val: bool) -> ColNull {
        ColNull {
            col: self.into(),
            is_null: val,
        }
    }
}
impl<T: Into<Col>> IntoCol for T {}

pub struct ColEq<T> {
    col: Col,
    val: T,
}

impl<'arg, T> Filter<'arg> for ColEq<T>
where
    T: 'arg + Encode<'arg, Postgres> + Type<Postgres> + Send,
{
    fn filter(self, builder: &mut QueryBuilder<'arg, Postgres>) {
        builder.push(format!(" {} = ", self.col));
        builder.push_bind(self.val);
    }
}

pub struct ColNull {
    col: Col,
    is_null: bool,
}

impl<'arg> Filter<'arg> for ColNull {
    fn filter(self, builder: &mut QueryBuilder<'arg, Postgres>) {
        let null = if self.is_null { "NULL" } else { "NOT NULL" };
        builder.push(format!(" {} IS {}", self.col, null));
    }
}

impl Filter<'_> for () {
    fn filter(self, _: &mut QueryBuilder<'_, Postgres>) {}
    fn effective(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Select<C, F = ()> {
    from: Iden,
    joins: Vec<Join>,
    filter: F,

    _pha: PhantomData<C>,
}

impl<C> Select<C> {
    pub(crate) fn new(name: Iden) -> Self {
        Self {
            from: name,
            joins: Default::default(),
            filter: (),
            _pha: PhantomData,
        }
    }

    pub fn join(mut self, ty: JoinTy, rel: RelationDef) -> Self {
        let join = Join {
            ty,
            tbl: rel.from_col.tbl.clone(),
            from_col: rel.from_col,
            to_col: rel.to_col,
        };
        self.joins.push(join);
        self
    }

    pub fn filter<F>(self, f: F) -> Select<C, F> {
        Select {
            from: self.from,
            joins: self.joins,
            filter: f,
            _pha: self._pha,
        }
    }
}

impl<'q, C: Selector, F: Filter<'q>> Select<C, F> {
    pub fn col<D>(self, _: D) -> Select<D, F> {
        Select {
            from: self.from,
            joins: self.joins,
            filter: self.filter,
            _pha: PhantomData,
        }
    }

    pub fn query(self) -> QueryBuilder<'q, Postgres> {
        let mut builder = QueryBuilder::new(format!("{}", self));
        if self.filter.effective() {
            builder.push("WHERE ");
            self.filter.filter(&mut builder);
        }
        builder
    }

    pub async fn one<'c, E>(self, e: E) -> Result<C::Data, sqlx::Error>
    where
        E: PgExecutor<'c>,
        C: Selector,
    {
        let mut query = self.query();
        let row = query.build().fetch_one(e).await?;
        C::from_row(&row)
    }

    pub async fn all<'c, E>(self, e: E) -> Result<Vec<C::Data>, sqlx::Error>
    where
        E: PgExecutor<'c>,
        C: Selector,
    {
        let mut query = self.query();

        query
            .build()
            .fetch(e)
            .map(|x| x.and_then(|x| C::from_row(&x)))
            .try_collect()
            .await
    }
}

impl<C, F> Display for Select<C, F>
where
    C: Selector,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = C::cols().join(", ");
        writeln!(f, "SELECT {} FROM {}", cols, self.from)?;
        for join in self.joins.iter().rev() {
            writeln!(f, "{}", join)?;
        }
        Ok(())
    }
}
