use super::*;

#[derive(Template)]
#[template(path = "table_index.askama", escape = "none")]
pub struct IndexWriter<'a> {
    schema: &'a IndexMap<String, TableDefinition>,
}

impl<'a> IndexWriter<'a> {
    pub fn generate(&self, root: &Path) -> Result<PathBuf, sqlx::Error> {
        let out = match self.render() {
            Ok(o) => Ok(o),
            Err(_) => Err(sqlx::Error::WorkerCrashed),
        }?;
        let path = root.join("mod.rs");
        std::fs::write(&path, out)?;
        Ok(path)
    }
}

impl PostgresExplorer {
    pub fn index_writer(&self) -> IndexWriter {
        IndexWriter { schema: &self.tables }
    }
}
