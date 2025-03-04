use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct AnalysisInfoEntity;

impl sqlx_orm::EntityTrait for AnalysisInfoEntity {}

#[derive(FromRow)]
pub struct AnalysisInfo {
    ///
    pub analysis_id: i64,
    ///
    pub project_id: Option<i64>,
    ///
    pub name: String,
    ///
    pub comment: String,
    ///
    pub analysis_function: String,
    ///
    pub analysis_parameters: sqlx::types::JsonValue,
    ///
    pub create_user: Option<i64>,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_user: Option<i64>,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub latest_queue: Option<i64>,
    /// 删除标记, 保留 7 天后才真正删除
    pub is_deleted: bool,
    ///
    pub custom: sqlx::types::JsonValue,
}

impl AnalysisInfo {
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
pub struct AnalysisMain;

impl AnalysisMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<AnalysisInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."analysis_info" WHERE "public"."analysis_info"."analysis_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<AnalysisInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."analysis_info" WHERE "public"."analysis_info"."analysis_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
