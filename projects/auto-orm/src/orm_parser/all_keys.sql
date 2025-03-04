SELECT tc.table_schema AS schema_name,
       tc.table_name,
       kcu.column_name,
       tc.constraint_name,
       tc.constraint_type
FROM information_schema.table_constraints tc
         JOIN information_schema.key_column_usage kcu
              ON tc.constraint_name = kcu.constraint_name
--            AND tc.table_schema = kcu.table_schema
--            AND tc.table_name = kcu.table_name
WHERE tc.constraint_type = 'UNIQUE'
   OR tc.constraint_type = 'PRIMARY KEY';