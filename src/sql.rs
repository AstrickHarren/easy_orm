use std::{fmt::Display, str::FromStr, sync::Arc};

use itertools::Itertools;
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    Encode, FromRow, PgExecutor, Postgres, QueryBuilder, Type,
};

use crate::relations::RelationDef;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PlaceHolder {
    index: u32,
}

impl Display for PlaceHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.index)
    }
}

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

pub(crate) struct Join {
    tbl: Iden,
    from_col: Col,
    to_col: Col,
}

impl Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JOIN {} ON {} = {}",
            self.tbl, self.from_col, self.to_col
        )
    }
}

pub trait Filter<'q>: Sized {
    fn filter(self, builder: &mut QueryBuilder<'q, Postgres>) {}
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

impl Filter<'_> for () {
    fn effective(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Select<F = ()> {
    cols: Vec<Col>,
    from: Iden,
    joins: Vec<Join>,
    filter: F,
}

impl Select {
    pub(crate) fn new(name: Iden) -> Self {
        Self {
            from: name,
            ..Default::default()
        }
    }

    pub fn col(mut self, col: impl IntoCol) -> Self {
        self.cols.push(col.into());
        self
    }

    pub fn join(mut self, rel: RelationDef) -> Self {
        let join = Join {
            tbl: rel.from_col.tbl.clone(),
            from_col: rel.from_col,
            to_col: rel.to_col,
        };
        self.joins.push(join);
        self
    }

    pub fn filter<F>(self, f: F) -> Select<F> {
        Select {
            cols: self.cols,
            from: self.from,
            joins: self.joins,
            filter: f,
        }
    }
}

impl<'q, F: Filter<'q>> Select<F> {
    pub fn query(self) -> QueryBuilder<'q, Postgres> {
        let mut builder = QueryBuilder::new(format!("{}", self));
        if self.filter.effective() {
            builder.push("WHERE ");
            self.filter.filter(&mut builder);
        }
        builder
    }

    pub async fn all<'c, T, E>(self, e: E) -> Result<Vec<T>, sqlx::Error>
    where
        E: PgExecutor<'c>,
        for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
    {
        let mut query = self.query();
        query.build_query_as().fetch_all(e).await
    }
}

impl<F> Display for Select<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = match self.cols.is_empty() {
            true => "*".to_string(),
            false => self.cols.iter().join(", "),
        };
        writeln!(f, "SELECT {} FROM {}", cols, self.from)?;
        for join in self.joins.iter().rev() {
            writeln!(f, "{}", join)?;
        }
        Ok(())
    }
}
