use super::*;

#[derive(Debug)]
pub struct TableDefinition {
    pub oid: Oid,
    pub name: String,
    pub comment: String,
    pub columns: Vec<ColumnDefinition>,
    pub primary_key: Option<Rc<KeyConstraintRow>>,
    pub unique_keys: Vec<Rc<KeyConstraintRow>>,
}

impl TableDefinition {
    pub fn uninitialized(name: &str) -> Result<Self, sqlx::Error> {
        Ok(Self {
            oid: Oid(0),
            name: name.to_string(),
            comment: "".to_string(),
            columns: Vec::new(),
            primary_key: None,
            unique_keys: vec![],
        })
    }
    pub fn unique_keys(&self) -> impl Iterator<Item = &KeyConstraintRow> {
        self.primary_key.iter().chain(self.unique_keys.iter()).map(|x| x.as_ref())
    }
    pub async fn initialize(&mut self, pool: &Pool<Postgres>) -> Result<String, sqlx::Error> {
        self.initialize_oid(pool).await?;
        Ok(self.name.clone())
    }
    pub async fn initialize_oid(&mut self, pool: &Pool<Postgres>) -> Result<Oid, sqlx::Error> {
        let oid: Option<Oid> = sqlx::query_scalar(
            "
            SELECT oid
            FROM pg_class
            WHERE relname = $1
            AND relkind = 'r'
            ",
        )
        .bind(&self.name)
        .fetch_one(pool)
        .await?;
        self.oid = oid.unwrap_or(Oid(0));
        Ok(self.oid)
    }
}
