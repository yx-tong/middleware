use super::*;

#[derive(FromRow)]
pub struct CommentRow {
    pub schema_name: String,
    pub table_name: String,
    pub column_name: Option<String>,
    pub comment: String,
}

impl PostgresExplorer {
    pub(crate) async fn query_comment(&mut self) -> Result<&[CommentRow], sqlx::Error> {
        let all: Vec<CommentRow> = sqlx::query_as(include_str!("all_comments.sql")).fetch_all(&self.pool).await?;
        self.comments = all;
        Ok(&self.comments)
    }
    pub(crate) fn update_comment(&mut self) {
        for table in self.tables.values_mut() {
            let parts: Vec<&CommentRow> = self
                .comments
                .iter()
                .filter(|x| x.schema_name.eq(&self.schema) && x.table_name.eq(table.name.as_str()))
                .collect();
            table.update_comment(&parts);
            table.columns.iter_mut().for_each(|column| column.update_comment(&parts));
        }
    }
}

impl TableDefinition {
    fn update_comment(&mut self, comments: &[&CommentRow]) {
        self.comment = comments
            .iter() //
            .find(|x| x.column_name.is_none())
            .map(|x| x.comment.clone())
            .unwrap_or_default();
    }
}

impl ColumnDefinition {
    fn update_comment(&mut self, comments: &[&CommentRow]) {
        self.comment = comments
            .iter()
            .find(|x| x.column_name.as_deref() == Some(&self.name))
            .map(|x| x.comment.clone())
            .unwrap_or_default();
    }
}
