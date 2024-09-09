#[cfg(feature = "sea-orm")]
use sea_orm::EntityTrait;

pub trait SemanticKey: Send + Sync {
    const KEY: &'static str;

    #[cfg(feature = "sea-orm")]
    type Entity: EntityTrait;
}
