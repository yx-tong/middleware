use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct ProjectVersionEntity;

impl sqlx_orm::EntityTrait for ProjectVersionEntity {}

#[derive(FromRow)]
pub struct ProjectVersion {
    ///
    pub version_id: i64,
    ///
    pub project_id: Option<i64>,
    ///
    pub name: String,
    ///
    pub group: String,
    ///
    pub comment: String,
    ///
    pub create_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub create_user: Option<i64>,
    ///
    pub update_time: sqlx::types::chrono::NaiveDateTime,
    ///
    pub update_user: Option<i64>,
    /// 删除标记, 保留 7 天后才真正删除
    pub is_deleted: bool,
    ///
    pub custom: sqlx::types::JsonValue,
}

impl ProjectVersion {
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
pub struct GameVersionMain;

impl GameVersionMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<ProjectVersion, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."project_version" WHERE "public"."project_version"."version_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<ProjectVersion>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."project_version" WHERE "public"."project_version"."version_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
