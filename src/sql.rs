use std::{fmt::Display, str::FromStr, sync::Arc};

use itertools::Itertools;
use sqlx::{postgres::PgArguments, query::Query, Encode, Postgres, QueryBuilder, Type};

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
pub(crate) struct Iden {
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
pub(crate) struct Col {
    tbl: Iden,
    col: Iden,
}

impl Col {
    pub(crate) fn new(tbl: Iden, col: Iden) -> Self {
        Self { tbl, col }
    }
}

impl Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.tbl, self.col)
    }
}

pub(crate) trait IntoCol: Into<Col> {
    fn eq<T>(self, val: T) -> ColEq<T> {
        ColEq {
            col: self.into(),
            val,
        }
    }
}
impl<T: Into<Col>> IntoCol for T {}

pub(crate) struct ColEq<T> {
    col: Col,
    val: T,
}

impl<'arg, T: Encode<'arg, Postgres> + Type<Postgres> + Clone + 'arg> Filter<'arg> for ColEq<T> {
    fn filter(self, builder: &mut QueryBuilder<'arg, Postgres>) -> () {
        builder.push(" ");
        builder.push(format!("{} = ", self.col));
        builder.push_bind(self.val);
    }
}

pub(crate) struct Join {
    tbl: Iden,
    from_col: Iden,
    to_col: Iden,
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

impl<'a> Filter<'a> for () {}

#[derive(Default)]
pub(crate) struct Select<F = ()> {
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

    pub(crate) fn filter<F>(self, filter: F) -> Select<F> {
        Select {
            cols: self.cols,
            from: self.from,
            joins: self.joins,
            filter,
        }
    }
}

impl<'arg, F: Filter<'arg> + 'arg> Select<F> {
    pub(crate) fn query(self) -> QueryBuilder<'arg, Postgres> {
        let mut builder = QueryBuilder::new(format!("{}", self));
        if self.filter.effective() {
            builder.push("WHERE ");
            self.filter.filter(&mut builder);
        }
        builder
    }
}

impl<F> Display for Select<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = match self.cols.is_empty() {
            true => "*".to_string(),
            false => self.cols.iter().join(", "),
        };
        writeln!(f, "SELECT {} FROM {}", cols, self.from)?;
        for join in &self.joins {
            writeln!(f, "{}", join)?;
        }
        Ok(())
    }
}
