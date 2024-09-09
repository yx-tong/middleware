use std::fmt::{Debug, Formatter};

#[cfg(feature = "sqlx")]
mod for_sqlx_pgsql;

#[cfg(feature = "poem-openapi")]
mod for_poem;

#[cfg(feature = "sea-orm")]
mod for_orm;

#[cfg(feature = "serde")]
mod for_serde;

#[derive(Clone, Default)]
pub struct ItemList<T> {
    pub list: Vec<T>,
}

impl<T: Debug> Debug for ItemList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.list, f)
    }
}

impl<T> IntoIterator for ItemList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl<T> ItemList<T> {
    pub fn min(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.list.iter().min()
    }
    pub fn max(&self) -> Option<&T>
    where
        T: Ord,
    {
        self.list.iter().max()
    }
}
