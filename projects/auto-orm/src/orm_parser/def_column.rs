use super::*;

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: String,
    pub comment: String,
    pub default_value: Option<String>,
    pub is_nullable: bool,
}

impl ColumnDefinition {
    pub fn is_option_type(&self) -> bool {
        if self.is_nullable {
            return true;
        }
        if self.default_value.is_some() {
            return true;
        }
        false
    }

    pub fn make_type(&self, buffer: &mut String) {
        match self.data_type.as_str() {
            "integer" => *buffer += "i32",
            "bigint" => *buffer += "i64",
            "double precision" => *buffer += "f64",
            "ARRAY" => *buffer += "Vec<sqlx::types::JsonValue>",
            // "bigint" => *buffer += "i64",
            // "bigint" => *buffer += "i64",
            // "bigint" => *buffer += "i64",
            // "bigint" => *buffer += "i64",
            // "bigint" => *buffer += "i64",
            // "bigint" => *buffer += "i64",
            "text" => *buffer += "String",
            "boolean" => *buffer += "bool",
            "jsonb" => *buffer += "sqlx::types::JsonValue",
            "uuid" => *buffer += "sqlx::types::Uuid",
            "timestamp without time zone" => *buffer += "sqlx::types::chrono::NaiveDateTime",
            other => {
                *buffer += other;
            }
        }
    }

    pub fn select_type(&self) -> String {
        let mut rust_type = String::new();
        if self.is_nullable {
            rust_type += "Option<";
        }
        self.make_type(&mut rust_type);
        if self.is_nullable {
            rust_type += ">";
        }
        rust_type
    }
}

#[derive(FromRow)]
pub struct InformationRow {
    table_name: String,
    column_name: String,
    data_type: String,
    column_default: Option<String>,
    is_nullable: String,
}

impl InformationRow {
    pub async fn query(schema: &str, pool: &Pool<Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
SELECT
    c.table_name,
    c.column_name,
    c.data_type,
    c.column_default,
    c.is_nullable
FROM
    information_schema.columns c
JOIN
    information_schema.tables t ON c.table_name = t.table_name
WHERE
    c.table_schema = $1 AND t.table_type = 'BASE TABLE'
ORDER BY
    c.table_name, c.ordinal_position
    "#,
        )
        .bind(schema)
        .fetch_all(pool)
        .await
    }

    pub fn split(self) -> (String, ColumnDefinition) {
        (self.table_name, ColumnDefinition {
            name: self.column_name,
            data_type: self.data_type,
            comment: "".to_string(),
            default_value: self.column_default,
            is_nullable: self.is_nullable == "YES",
        })
    }
}
