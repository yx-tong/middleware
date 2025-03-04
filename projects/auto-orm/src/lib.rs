#![feature(try_trait_v2)]
#![feature(adt_const_params)]
#![feature(duration_millis_float)]

pub mod helpers;
mod orm_generate;
mod orm_parser;
mod orm_traits;
mod orm_write;

pub use crate::{
    orm_parser::{PostgresExplorer, def_column::ColumnDefinition, def_table::TableDefinition},
    orm_traits::EntityTrait,
    orm_write::{wri_index::IndexWriter, wri_table::TableWriter},
};

#[cfg(test)]
mod test {
    use crate::PostgresExplorer;
    use std::path::Path;

    #[tokio::test]
    async fn main() {
        let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/examples");
        let mut pool = PostgresExplorer::new(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
        pool.get_definitions().await.unwrap();
        for table in pool.table_writer() {
            table.generate(&here).unwrap();
        }
        pool.index_writer().generate(&here).unwrap();
    }
}
