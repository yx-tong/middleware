use crate::errors::{YxError, YxResult};
use crate::ApiEndpoint;
use api_result::{Failure, Success};
use orm::insurance::sea_orm_active_enums::Gender;
use orm::UserCreation;
use poem_openapi::{Object};
use sea_orm::prelude::{Date, Json};
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, FromQueryResult, IntoActiveModel, QueryFilter, QuerySelect};
use uuid::Uuid;


#[derive(Object)]
pub struct GetConsumerList {
    /// 业务员 uuid
    seller_id: Uuid,
    /// 页数, 从 0 开始
    page: Option<u64>,
    /// 不超过 100 个
    limit: Option<u64>,
}

#[derive(Object)]
pub struct CreateConsumer {
    /// 业务员 uuid
    seller_id: Uuid,
}

/// 新创建的客户
#[derive(Object)]
pub struct NewConsumer {
    /// 业务员 uuid
    seller_id: Uuid,
    /// 客户 uuid
    consumer_id: Uuid,
}

#[derive(Object)]
pub struct GetConsumer {
    /// 客户 uuid
    consumer_id: Uuid,
}

/// 更新客户数据, 只有非 null 字段会更新
#[derive(Object)]
pub struct SetConsumer {
    /// 客户 uuid
    consumer_id: Uuid,
    /// 业务员 id
    seller_id: Option<Uuid>,
    /// 客户名字
    name: Option<String>,
    /// 客户性别
    gender: Option<Gender>,
    /// 客户生日
    birthday: Option<chrono::naive::NaiveDate>,
    /// 客户公司
    company: Option<String>,
    /// 客户职业
    job: Option<String>,
    /// 客户职位
    career: Option<String>,
    /// 家庭情况
    family: Option<serde_json::Value>,
    /// 收入情况
    income: Option<serde_json::Value>,
    /// 房屋情况
    houses: Option<serde_json::Value>,
    /// 车辆状况
    car: Option<serde_json::Value>,
    /// 健康状况
    health_status: Option<serde_json::Value>,
    /// 保险意愿
    insurance_status: Option<serde_json::Value>,
    /// 保险状况
    insurance_willingness: Option<serde_json::Value>,
    /// 额外信息
    consumer_extra: Option<serde_json::Value>,
}

#[derive(Object, FromQueryResult)]
pub struct ConsumerData {
    /// 客户 uuid
    pub consumer_id: Uuid,
    /// 业务员 uuid
    pub seller_id: Uuid,
    /// 客户名字
    pub name: String,
    /// 客户性别
    pub gender: Gender,
    /// 客户生日
    pub birthday: Date,
    /// 客户公司
    pub company: String,
    pub job: String,
    pub career: String,
    pub family: Json,
    pub income: Json,
    pub houses: Json,
    pub car: Json,
    pub health_status: Json,
    pub insurance_willingness: Json,
    pub insurance_status: Json,
    pub consumer_extra: Json,
}

#[derive(Object, FromQueryResult)]
pub struct ConsumerDataPreview {
    consumer_id: Uuid,
    seller_id: Uuid,
    name: String,
    gender: Gender,
}

impl GetConsumerList {
    pub async fn execute(self, db: &ApiEndpoint) -> YxResult<Vec<ConsumerDataPreview>> {
        let tx = db.transaction().await?;
        let limit = match self.limit {
            None => { 20 }
            Some(s) if s > 100 => {
                100
            }
            Some(s) => s
        };
        let page = self.page.unwrap_or(0);
        let offset = page * limit;

        let a = orm::insurance::consumers::Entity::find()
            .filter(orm::insurance::consumers::Column::SellerId.eq(self.seller_id))
            .offset(offset)
            .limit(limit)
            .into_model::<ConsumerDataPreview>()
            .all(&tx)
            .await?
            ;
        tx.commit().await?;
        Success(a)
    }
}


impl CreateConsumer {
    pub async fn execute(self, db: &ApiEndpoint) -> YxResult<NewConsumer> {
        let tx = db.transaction().await?;
        let consumer = tx.create_insurance_consumer_with_salesman(self.seller_id).await?;
        tx.commit().await?;
        Success(NewConsumer {
            seller_id: self.seller_id,
            consumer_id: consumer,
        })
    }
}


impl GetConsumer {
    pub async fn execute(self, db: &ApiEndpoint) -> YxResult<ConsumerData> {
        let tx = db.transaction().await?;
        let data = orm::insurance::consumers::Entity::find()
            .filter(orm::insurance::consumers::Column::ConsumerId.eq(self.consumer_id))
            .into_model::<ConsumerData>()
            .one(&tx)
            .await?
            ;
        tx.commit().await?;
        match data {
            Some(s) => {
                Success(s)
            }
            None => {
                Failure(
                    YxError::database_error(format!("Consumer `{}` not found", self.consumer_id))
                )
            }
        }
    }
}
impl SetConsumer {
    pub async fn execute(self, db: &ApiEndpoint) -> YxResult<bool> {
        let tx = db.transaction().await?;
        let consumer = orm::insurance::consumers::Entity::find_by_id(self.consumer_id)
            .one(&tx)
            .await?
            .map(|v| v.into_active_model());
        let mut consumer = match consumer {
            Some(s) => { s }
            None => {
                return Success(false)
            }
        };
        if let Some(s) = self.seller_id {
            consumer.seller_id = ActiveValue::Set(s)
        }
        if let Some(s) = self.name {
            consumer.name = ActiveValue::Set(s)
        }
        if let Some(s) = self.gender {
            consumer.gender = ActiveValue::Set(s)
        }
        if let Some(s) = self.birthday {
            consumer.birthday = ActiveValue::Set(s)
        }
        if let Some(s) = self.company {
            consumer.company = ActiveValue::Set(s)
        }
        if let Some(s) = self.job {
            consumer.job = ActiveValue::Set(s)
        }

        if let Some(s) = self.family {
            consumer.family = ActiveValue::Set(s)
        }
        if let Some(s) = self.income {
            consumer.income = ActiveValue::Set(s)
        }
        if let Some(s) = self.houses {
            consumer.houses = ActiveValue::Set(s)
        }
        if let Some(s) = self.car {
            consumer.car = ActiveValue::Set(s)
        }
        if let Some(s) = self.consumer_extra {
            consumer.consumer_extra = ActiveValue::Set(s)
        }
        if let Some(s) = self.health_status {
            consumer.health_status = ActiveValue::Set(s)
        }
        if let Some(s) = self.insurance_willingness {
            consumer.insurance_willingness = ActiveValue::Set(s)
        }
        if let Some(s) = self.insurance_status {
            consumer.insurance_status = ActiveValue::Set(s)
        }
        consumer.update(&tx).await?;
        tx.commit().await?;
        Success(true)
    }
}


