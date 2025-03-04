use crate::{
    ColumnDefinition, TableDefinition,
    orm_parser::{def_column::InformationRow, def_comment::CommentRow, def_keys::KeyConstraintRow},
};
use indexmap::IndexMap;
use sqlx::{
    FromRow, Pool, Postgres,
    postgres::{PgPoolOptions, types::Oid},
};
use std::rc::Rc;

pub mod def_column;
pub mod def_comment;
pub mod def_keys;
pub mod def_table;

pub struct PostgresExplorer {
    pub pool: Pool<Postgres>,
    pub schema: String,
    pub tables: IndexMap<String, TableDefinition>,
    pub keys: IndexMap<String, Rc<KeyConstraintRow>>,
    pub comments: Vec<CommentRow>,
}

impl PostgresExplorer {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Self { pool, schema: "public".to_string(), tables: Default::default(), keys: Default::default(), comments: vec![] })
    }
}

impl PostgresExplorer {
    pub async fn get_definitions(&mut self) -> Result<(), sqlx::Error> {
        let rows = InformationRow::query(&self.schema, &self.pool).await?;
        self.query_comment().await?;
        self.query_keys().await?;
        for row in rows {
            let (table_name, column) = row.split();
            if !self.tables.contains_key(&table_name) {
                let mut table = TableDefinition::uninitialized(&table_name)?;
                let name = table.initialize(&self.pool).await?;
                self.tables.insert(name, table);
            }
            self.tables.get_mut(&table_name).unwrap().columns.push(column);
        }
        self.update_comment();
        self.update_keys();
        Ok(())
    }
}
