use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct EventLogAggregationEntity;

impl sqlx_orm::EntityTrait for EventLogAggregationEntity {}

pub struct EventLogAggregation {
    //
    pub aggregation_id: tsrange,

    //
    pub range: Option<tsrange>,
}

#[automatically_derived]
impl<'a, R: Row> FromRow<'a, R> for EventLogAggregation {
    fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
        Ok(Self { aggregation_id: row.try_get("aggregation_id")?, range: row.try_get("range")? })
    }
}

impl EventLogAggregation {
    pub async fn find_by(key: String, db: &Pool<Postgres>) -> Result<Self, Error> {
        sqlx::query_as(r#"SELECT * FROM public.advertisement_info WHERE advertise_id = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn find_many(db: &Pool<Postgres>, limit: u64) -> Result<Vec<Self>, Error> {
        sqlx::query_as(r#"SELECT * FROM public.advertisement_info LIMIT $1"#).bind(limit as i64).fetch_all(db).await
    }
}
