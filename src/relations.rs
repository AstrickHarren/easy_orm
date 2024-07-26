use crate::sql::Col;

#[derive(Debug, Default)]
pub struct RelationBuilder {
    from_col: Option<Col>,
    to_col: Option<Col>,
}

impl RelationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(mut self, col: impl Into<Col>) -> Self {
        self.from_col = Some(col.into());
        self
    }

    pub fn to(mut self, col: impl Into<Col>) -> Self {
        self.to_col = Some(col.into());
        self
    }
}

impl From<RelationBuilder> for RelationDef {
    fn from(val: RelationBuilder) -> Self {
        RelationDef {
            from_col: val.from_col.unwrap(),
            to_col: val.to_col.unwrap(),
        }
    }
}

/// For example, if the following tables are created:
/// cake(id), cake_filling(cake_id, filling_id)
///
/// Then the relation cake.id --> cake_filling.cake_id belongs to `cake`
/// since the other column is a referernce to the column in `cake`
pub struct RelationDef {
    pub from_col: Col,
    pub to_col: Col,
}

impl RelationDef {
    pub fn rev(self) -> Self {
        Self {
            from_col: self.to_col,
            to_col: self.from_col,
        }
    }
}

pub trait Related<E> {
    fn to() -> RelationDef;
    fn via() -> Option<RelationDef> {
        None
    }
}

pub trait RelationTrait {
    fn def(&self) -> RelationDef;
}
