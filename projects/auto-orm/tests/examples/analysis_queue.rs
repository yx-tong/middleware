use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct AnalysisQueueEntity;

impl sqlx_orm::EntityTrait for AnalysisQueueEntity {}

#[derive(FromRow)]
pub struct AnalysisQueue {
    ///
    pub queue_id: sqlx::types::Uuid,
    ///
    pub analysis_id: sqlx::types::Uuid,
    ///
    pub create_user: sqlx::types::Uuid,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub inputs: sqlx::types::JsonValue,
    ///
    pub results: sqlx::types::JsonValue,
    ///
    pub result_status: i32,
    ///
    pub result_message: String,
    ///
    pub result_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub cost: f64,
}

impl AnalysisQueue {
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
pub struct AnalysisQueuePkey;

impl AnalysisQueuePkey {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<AnalysisQueue, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."analysis_queue" WHERE "public"."analysis_queue"."queue_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<AnalysisQueue>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."analysis_queue" WHERE "public"."analysis_queue"."queue_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
