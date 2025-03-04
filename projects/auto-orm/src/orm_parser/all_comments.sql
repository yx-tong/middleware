SELECT n.nspname     as schema_name,
       c.relname     as table_name,
       NULL          as column_name,
       d.description as comment
FROM pg_class c
         JOIN pg_namespace n ON n.oid = c.relnamespace
         LEFT JOIN pg_description d ON d.objoid = c.oid AND d.objsubid = 0
WHERE true
--   AND c.relkind = 'r'
  AND n.nspname NOT LIKE 'pg_%'
  AND n.nspname != 'information_schema'
  AND d.description is not null

UNION ALL

SELECT n.nspname     as schema_name,
       c.relname     as table_name,
       a.attname     as column_name,
       d.description as comment
FROM pg_class c
         JOIN pg_namespace n ON n.oid = c.relnamespace
         JOIN pg_attribute a ON a.attrelid = c.oid
         LEFT JOIN pg_description d ON d.objoid = c.oid AND d.objsubid = a.attnum
WHERE true
--   AND c.relkind = 'r'
  AND n.nspname NOT LIKE 'pg_%'
  AND n.nspname != 'information_schema'
--   AND a.attnum > 0
--   AND NOT a.attisdropped
  AND d.description is not null
;