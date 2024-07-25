use crate::sql::{self, Col};

#[derive(Debug, Default)]
pub(crate) struct RelationBuilder {
    from_col: Option<Col>,
    to_col: Option<Col>,
}

impl RelationBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn from(mut self, col: impl Into<Col>) -> Self {
        self.from_col = Some(col.into());
        self
    }

    pub(crate) fn to(mut self, col: impl Into<Col>) -> Self {
        self.to_col = Some(col.into());
        self
    }
}

impl Into<RelationDef> for RelationBuilder {
    fn into(self) -> RelationDef {
        RelationDef {
            from_col: self.from_col.unwrap(),
            to_col: self.to_col.unwrap(),
        }
    }
}

/// For example, if the following tables are created:
/// cake(id), cake_filling(cake_id, filling_id)
///
/// Then the relation cake.id --> cake_filling.cake_id belongs to `cake`
/// since the other column is a referernce to the column in `cake`
pub(crate) struct RelationDef {
    from_col: Col,
    to_col: Col,
}

pub(crate) trait Related<E> {
    fn to() -> RelationDef;
}

pub(crate) trait RelationTrait {
    fn def(&self) -> RelationDef;
}
