use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct LevelInfoEntity;

impl sqlx_orm::EntityTrait for LevelInfoEntity {}

#[derive(FromRow)]
pub struct LevelInfo {
    ///
    pub level_id: sqlx::types::Uuid,
    ///
    pub project_id: sqlx::types::Uuid,
    ///
    pub name: String,
    ///
    pub comment: String,
    /// 等级组
    pub group: Option<String>,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub create_user: sqlx::types::Uuid,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_user: sqlx::types::Uuid,
    ///
    pub is_deleted: bool,
    ///
    pub custom: sqlx::types::JsonValue,
}

impl LevelInfo {
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
pub struct LevelMain;

impl LevelMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<LevelInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."level_info" WHERE "public"."level_info"."level_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<LevelInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."level_info" WHERE "public"."level_info"."level_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
pub struct LevelInfoPk;

impl LevelInfoPk {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<LevelInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."level_info" WHERE "public"."level_info"."group" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<LevelInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."level_info" WHERE "public"."level_info"."group" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
