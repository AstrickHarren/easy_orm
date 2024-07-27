#[macro_export]
macro_rules! data_table {
    ($model:ident of $table_name:ident {
        $([$id_col:ident: $id_ty:ty],)?
        $($col:ident : $col_ty:ty $(=> $ref:ident.$ref_col:ident)?),* $(,)?
    }) => {
        paste::paste!{
        #[allow(unused_imports)]
        pub use [<$model:snake>]::{
            Entity as $model,
            Insert as [<Insert $model>],
            Update as [<Update $model>]
        };
        pub mod [<$model:snake>] {
            #![allow(unused_imports)]
            #![allow(dead_code)]
            use super::*;
            use $crate::common::{EntityTrait, ColumnList};
            use $crate::relations::{ RelationTrait, RelationDef, RelationBuilder, Related };
            use $crate::sql::{Col, Select};
            use sqlx::{ QueryBuilder, Postgres, PgExecutor, Error, FromRow };

            macro_rules! idem_option {
                (Option<$ty:ty>) => {
                    Option<$ty>
                };
                ($ty:ty) => {
                    Option<$ty>
                }
            }

            pub struct Entity;

            impl EntityTrait for Entity {
                type Row = Row;
                type Column = Column;
                const TABLE_NAME: &'static str = stringify!($table_name);
            }

            impl ColumnList for Entity {
                type Data = Row;
                fn cols() -> impl Iterator<Item = Col> {
                    std::iter::once(Col {
                        tbl: Self::TABLE_NAME.into(),
                        col: "*".into()
                    })
                }
            }

            #[derive(Debug, FromRow)]
            pub struct Row {
                $(pub $id_col: $id_ty,)?
                $(pub $col: $col_ty,)*
            }

            #[derive(Debug, Default)]
            pub struct Update {
                $($id_col: $id_ty,)?
                $($col: idem_option!($col_ty),)*
            }

            impl Update {
                $(
                pub fn new($id_col: $id_ty) -> Self {
                    Self::__new($id_col)
                }
                )?

                fn __new($($id_col: $id_ty)?) -> Self {
                    Self {
                        $($id_col,)?
                        $($col: None,)*
                    }
                }

                $(
                pub fn $col(mut self, val: impl Into<$col_ty>) -> Self
                {
                    let val: $col_ty = val.into();
                    self.$col = val.into();
                    self
                }
                )*

                pub fn query(&self) -> QueryBuilder<Postgres> {
                    let sql = format!("UPDATE {} SET ", Entity::TABLE_NAME);
                    let mut builder = QueryBuilder::<Postgres>::new(sql);
                    let mut sep = builder.separated(", ");
                    $(
                        sep.push(format!("{} = ", stringify!($col)));
                        sep.push_bind_unseparated(&self.$col);
                    )*
                    $(
                        builder.push(format!(" WHERE {} = ", stringify!($id_col)));
                        builder.push_bind(self.$id_col);
                    )?
                    builder
                }
            }

            #[derive(Debug, Default)]
            pub struct Insert {
                $(pub $col: $col_ty,)*
            }

            macro_rules! cond {
                ($body:tt; then $then:tt; else $else:tt ) => { $then };
                (then; else $else:tt) => { $else }
            }

            // IdTy is `$id_ty` if `$id_col` is present, otherwise '()'
            type IdTy = cond!( $($id_col;)? then $($id_ty)?; else ());

            impl Insert {
                pub fn insert_query(&self) -> QueryBuilder<Postgres> {
                    let cols = stringify!($($col),*);
                    let sql = format!("INSERT INTO {} ({}) VALUES (", Entity::TABLE_NAME, cols);
                    let mut builder = QueryBuilder::<Postgres>::new(sql);
                    let mut sep = builder.separated(", ");
                    $(sep.push_bind(&self.$col);)*
                    sep.push_unseparated(")");
                    builder
                }

                pub async fn insert<'c, E: PgExecutor<'c>>(&self, e: E) -> Result<IdTy, Error>{
                    let mut query = self.insert_query();
                    $(query.push(format!(" RETURNING {}", stringify!($id_col)));)?
                    query.build_query_scalar().fetch_one(e).await
                }
            }

            pub enum Column {
                $( [<$id_col:camel>],)?
                $( [<$col:camel>],)*
            }

            impl Into<Col> for Column {
                fn into(self) -> Col {
                    match self {
                        $(Column::[<$id_col:camel>] => Col::new(
                            Entity::TABLE_NAME.into(), stringify!($id_col).into())
                        ,)?
                        $(Column::[<$col:camel>] => Col::new(
                            Entity::TABLE_NAME.into(), stringify!($col).into())
                        ,)*
                    }
                }
            }

            pub enum Relation {
                $($([<$ref:camel>],)?)*
                Nothing
            }

            impl RelationTrait for Relation {
                fn def(&self) -> RelationDef {
                    match self {
                        $($(
                        Relation::[<$ref:camel>] => RelationBuilder::new()
                            .from(Column::[<$col:camel>])
                            .to([<$ref:snake>]::Column::[<$ref_col:camel>])
                            .into(),
                        )?)*
                        Relation::Nothing => unreachable!()
                    }
                }
            }

            $($(
            impl Related<[<$ref:snake>]::Entity> for Entity {
                fn to() -> RelationDef {
                    Relation::[<$ref:camel>].def()
                }
            }

            impl Related<Entity> for [<$ref:snake>]::Entity {
                fn to() -> RelationDef {
                    Relation::[<$ref:camel>].def().rev()
                }
            }
            )?)*
        }
        }
    };
}

#[macro_export]
macro_rules! many_to_many {
    ($from:ident -> $via:ident -> $to:ident) => {
        paste::paste! {
        impl $crate::relations::Related<$to::Entity> for $from::Entity {
            fn to() -> $crate::relations::RelationDef {
                use $crate::relations::RelationTrait;
                $via::Relation::[<$to:camel>].def()
            }

            fn via() -> Option<$crate::relations::RelationDef> {
                use $crate::relations::RelationTrait;
                $via::Relation::[<$from:camel>].def().rev().into()
            }
        }
        }
    };
    ($from:ident - $via:ident - $to:ident) => {
        many_to_many!($from -> $via -> $to);
        many_to_many!($to -> $via -> $from);
    };
}

#[cfg(test)]
mod tests {
    use crate::common::EntityTrait;

    data_table!(Person of people {
        [id: i32],
        name: String,
        addr: Option<String>,
        age: Option<i32>,
    });

    data_table!(Circle of circles {
        [id: i32],
        name: String,
    });

    data_table!(PersonCircle of person_circle {
        person_id: i32 => Person.id,
        circle_id: i32 => Circle.id,
    });

    many_to_many!(person - person_circle - circle);

    #[test]
    fn test() {
        println!("{}", Person::find_related::<PersonCircle>().query().sql());
        println!("{}", PersonCircle::find_related::<Person>().query().sql());
        println!("{}", Person::find_related::<Circle>().query().sql());

        let query = InsertPerson {
            name: "Nir".to_string(),
            ..Default::default()
        };
        println!("{}", query.insert_query().into_sql());

        let update = person::Update::new(1)
            .name("Gil".to_string())
            .addr("5000 Forbes".to_string())
            .age(10)
            .query()
            .into_sql();
        println!("{}", update);
    }
}
