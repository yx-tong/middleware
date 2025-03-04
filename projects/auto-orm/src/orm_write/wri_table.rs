use super::*;

#[derive(Template)]
#[template(path = "table_schema.askama", escape = "none")]
pub struct TableWriter<'a> {
    table: &'a TableDefinition,
}

impl<'a> TableWriter<'a> {
    pub fn file_name(&self) -> String {
        format!("{}.rs", self.table.name.to_lowercase())
    }
    pub fn read_name(&self) -> String {
        self.table.name.to_upper_camel_case()
    }
    pub fn entity_name(&self) -> String {
        format!("{}Entity", self.table.name.to_upper_camel_case())
    }

    pub fn generate(&self, root: &Path) -> Result<PathBuf, sqlx::Error> {
        let out = match self.render() {
            Ok(o) => Ok(o),
            Err(_) => Err(sqlx::Error::WorkerCrashed),
        }?;
        let path = root.join(self.file_name());
        std::fs::write(&path, out)?;
        Ok(path)
    }
}

impl PostgresExplorer {
    pub fn table_writer(&self) -> impl Iterator<Item = TableWriter> {
        self.tables.values().map(|table| TableWriter { table })
    }
}
