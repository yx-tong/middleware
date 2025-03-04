use super::*;
use heck::ToUpperCamelCase;

#[derive(Debug, FromRow)]
pub struct KeyConstraintRow {
    pub schema_name: String,
    pub table_name: String,
    pub column_name: String,
    pub constraint_name: String,
    pub constraint_type: String,
}

impl KeyConstraintRow {
    pub fn is_unique_key(&self) -> bool {
        self.constraint_type == "UNIQUE"
    }

    pub fn struct_name(&self) -> String {
        self.constraint_name.to_upper_camel_case()
    }

    pub fn full_table(&self) -> String {
        format!("{:?}.{:?}", self.schema_name, self.table_name)
    }

    pub fn full_column(&self) -> String {
        format!("{:?}.{:?}.{:?}", self.schema_name, self.table_name, self.column_name)
    }

    pub fn is_primary_key(&self) -> bool {
        self.constraint_type == "PRIMARY KEY"
    }
}

impl PostgresExplorer {
    pub(crate) async fn query_keys(&mut self) -> Result<(), sqlx::Error> {
        let keys: Vec<KeyConstraintRow> = sqlx::query_as(include_str!("all_keys.sql")).fetch_all(&self.pool).await?;
        self.keys = keys.into_iter().map(|k| (k.constraint_name.clone(), Rc::new(k))).collect();
        Ok(())
    }
    pub(crate) fn update_keys(&mut self) {
        for table in self.tables.values_mut() {
            let keys: Vec<_> = self //
                .keys
                .values()
                .filter(|x| x.schema_name.eq(&self.schema) && x.table_name.eq(table.name.as_str()))
                .map(Rc::clone)
                .collect();
            table.update_keys(&keys)
        }
    }
}

impl TableDefinition {
    fn update_keys(&mut self, keys: &[Rc<KeyConstraintRow>]) {
        self.primary_key = keys.iter().filter(|x| x.is_primary_key()).map(Rc::clone).next();
        self.unique_keys = keys.iter().filter(|x| x.is_unique_key()).map(Rc::clone).collect();
    }
}
