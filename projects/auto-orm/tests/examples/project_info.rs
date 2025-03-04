use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct ProjectInfoEntity;

impl sqlx_orm::EntityTrait for ProjectInfoEntity {}

#[derive(FromRow)]
pub struct ProjectInfo {
    ///
    pub project_id: i64,
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
    /// 项目状态, 0 表示正常, 其他值表示不同状态
    pub status: i32,
    ///
    pub custom: sqlx::types::JsonValue,
    ///
    pub is_deleted: bool,
}

impl ProjectInfo {
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
pub struct ProjectMain;

impl ProjectMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<ProjectInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."project_info" WHERE "public"."project_info"."project_id" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<ProjectInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."project_info" WHERE "public"."project_info"."project_id" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
