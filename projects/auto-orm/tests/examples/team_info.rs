use sqlx::{Error, FromRow, Pool, Postgres, Row};

pub struct TeamInfoEntity;

impl sqlx_orm::EntityTrait for TeamInfoEntity {}

#[derive(FromRow)]
pub struct TeamInfo {
    ///
    pub team: i64,
    ///
    pub super_team: Option<i64>,
    ///
    pub permissions_inherit: Vec<sqlx::types::JsonValue>,
    ///
    pub permissions: Vec<sqlx::types::JsonValue>,
}

impl TeamInfo {
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
pub struct TeamMain;

impl TeamMain {
    pub async fn read_one(key: sqlx::types::Uuid, db: &Pool<Postgres>) -> Result<TeamInfo, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."team_info" WHERE "public"."team_info"."team" = $1 LIMIT 1"#)
            .bind(key)
            .fetch_one(db)
            .await
    }
    pub async fn read_all(key: &[sqlx::types::Uuid], db: &Pool<Postgres>) -> Result<Vec<TeamInfo>, Error> {
        sqlx::query_as(r#"SELECT * FROM "public"."team_info" WHERE "public"."team_info"."team" = ANY($1)"#)
            .bind(key)
            .fetch_all(db)
            .await
    }
}
