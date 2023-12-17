-- insert into mares (name, breed)
-- values ('Fluttershy', 1)

-- select id as "id!", name as "name!", breed as "breed!"
-- from mares
-- where id = 3;

select *
from mares;


-- if record with id exists {
--     if modified_at == record.modified_at {
--         update record
--     } else {
--         return error: modified_at_conflict
--     }
-- } else {
--     return error: record_not_found
-- }


