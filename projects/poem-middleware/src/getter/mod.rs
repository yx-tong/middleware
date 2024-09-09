use poem_openapi::types::ToJSON;
use sea_orm::{ConnectionTrait, DbBackend, DbErr, ExecResult, FromQueryResult, QueryResult, Statement, TryGetable, Value};

#[derive(Debug)]
pub struct SqlBuilder {
    sql: String,
    inputs: Vec<Value>,
}

impl SqlBuilder {
    pub fn new(sql: impl Into<String>) -> Self {
        Self { sql: sql.into(), inputs: vec![] }
    }
    pub fn arg<T>(mut self, value: T) -> Self
    where
        T: Into<Value>,
    {
        self.inputs.push(value.into());
        self
    }
    pub fn json<T>(self, value: T) -> Self
    where
        T: ToJSON,
    {
        self.arg(Value::Json(value.to_json().map(Box::new)))
    }
    pub fn args<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        self.inputs.extend(values.into_iter());
        self
    }
    pub fn finish(self) -> Statement {
        Statement::from_sql_and_values(DbBackend::Postgres, self.sql, self.inputs)
    }

    pub async fn execute(self, db: &impl ConnectionTrait) -> Result<ExecResult, DbErr> {
        db.execute(self.finish()).await
    }

    pub async fn query_optional(self, db: &impl ConnectionTrait) -> Result<Option<QueryResult>, DbErr> {
        db.query_one(self.finish()).await
    }
    pub async fn query_one(self, db: &impl ConnectionTrait) -> Result<QueryResult, DbErr> {
        match db.query_one(self.finish()).await? {
            Some(s) => Ok(s),
            None => Err(DbErr::RecordNotFound("AnonymousSQL".to_string())),
        }
    }
    pub async fn query_one_as<T>(self, db: &impl ConnectionTrait) -> Result<T, DbErr>
    where
        T: TryGetable,
    {
        self.query_one(db).await?.try_get_by_index(0)
    }
    pub async fn query_all(self, db: &impl ConnectionTrait) -> Result<Vec<QueryResult>, DbErr> {
        db.query_all(self.finish()).await
    }
    pub async fn query_all_as<T>(self, db: &impl ConnectionTrait) -> Result<Vec<T>, DbErr>
    where
        T: FromQueryResult,
    {
        self.query_all(db).await?.into_iter().map(|x| T::from_query_result(&x, "")).collect()
    }
}
