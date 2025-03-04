use sqlparser::{
    ast::{
        Expr, GroupByExpr, Ident, ObjectName, Query, Select, SelectItem, SelectItem::UnnamedExpr, SetExpr, Statement, Table,
        TableFactor, TableWithJoins,
    },
    dialect::GenericDialect,
    parser::Parser,
};

pub struct Namepath {
    path: Vec<String>,
}

pub struct TakeField {
    table: Namepath,
    fields: Vec<String>,
}

impl TakeField {
    pub fn as_projection(&self) -> Vec<SelectItem> {
        self.fields.iter().map(|x| UnnamedExpr(Expr::Identifier(Ident::new(x)))).collect()
    }
}

impl Namepath {
    pub fn as_object_name(&self) -> ObjectName {
        ObjectName(self.path.iter().map(|x| Ident::new(x)).collect())
    }
    pub fn safe_name(&self, raw: String) -> String {
        self.path.join(".")
    }
}

#[test]
fn test() {
    let sql = r#"
SELECT a, b, 123, myfunc(b) 
    FROM table_1 
    WHERE a > b AND b < 100 
    ORDER BY a DESC, b
           "#;

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    // public.table
    let vq = TakeField {
        table: Namepath { path: vec!["public".to_string(), "where".to_string(), "group".to_string()] },
        fields: vec!["*".to_string(), "a".to_string()],
    };

    let table = Table { table_name: Some("what".to_string()), schema_name: Some("public".to_string()) };

    let s = Statement::Query(Box::new(Query {
        with: None,
        body: Box::new(SetExpr::Select(Box::new(Select {
            distinct: None,
            top: None,
            projection: vq.as_projection(),
            into: None,
            from: vec![TableWithJoins {
                relation: TableFactor::Table {
                    name: vq.table.as_object_name(),
                    alias: None,
                    args: None,
                    with_hints: vec![],
                    version: None,
                    with_ordinality: false,
                    partitions: vec![],
                },
                joins: vec![],
            }],
            lateral_views: vec![],
            prewhere: None,
            selection: None,
            group_by: GroupByExpr::Expressions(vec![], vec![]),
            cluster_by: vec![],
            distribute_by: vec![],
            sort_by: vec![],
            having: None,
            named_window: vec![],
            qualify: None,
            window_before_qualify: false,
            value_table_mode: None,
            connect_by: None,
        }))),
        order_by: None,
        limit: None,
        limit_by: vec![],
        offset: None,
        fetch: None,
        locks: vec![],
        for_clause: None,
        settings: None,
        format_clause: None,
    }));
    println!("SQL: {:#?}", s.to_string());
    for x in ast {
        println!("AST: {:#?}", x)
    }
}
