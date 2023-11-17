-- SQLite
-- create table test_mares (
--        id integer primary key autoincrement not null,
--      name varchar not null,
--     breed integer not null,
--     timestamp timestamp not null
-- );

-- create table test (
--      name varchar,
--     breed integer
-- );

-- 0 - unicorn, 1 - earth, 2 - pegasus



-- ("Twilight Sparkle", 0),
-- ("Applejack", 1);


-- insert into mares (name, breed)
-- values ("Applejack", 1);

-- create unique index index2 ON mares(name) where mares(breed) = 2;

-- alter mares (

-- explain query plan
-- select rowid as "row id", *
-- from mares;
-- where name > "Fluttershy";

-- delete from mares
-- where id = 2
-- returning name, breed;
-- explain query plan


-- update mares
-- set breed = 1, name = '3 EDIT'
-- where id = 44
-- returning (select name from mares where id = 44)

-- insert into test_mares (name, breed, timestamp)
-- values ('Fluttershy', 1, 1000);


-- alter table test_mares
-- add modified_at datetime null;

-- update test_mares
-- set modified_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW');


-- alter table test_mares
-- drop column modified_at;

-- ALTER TABLE MY_TABLE ADD STAGE INT NOT NULL DEFAULT '0'

-- alter table test_mares
-- add modified_at datetime not null;

-- with values STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW');


-- insert into test_mares (name, breed, modified_at)
-- values ('Rarity', 2, STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'));

-- PRAGMA table_info(test_mares);

-- alter table test_mares
-- alter column modified_at datetime not null;



-- ALTER TABLE test_mares ADD modified_at datetime not null DEFAULT 0;

-- alter table test_mares drop column modified_at;

-- alter table test_mares add column modified_at datetime not null default 0;
-- update test_mares set modified_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW');

-- alter table mares
-- drop column modified_at;

-- alter table mares
-- add column modified_at timestamp default 0 not null ;

-- update mares
-- set modified_at = strftime('%s','now');

select *
from mares;

-- update mares
-- set name = 'Rainbowshine', breed = 1, modified_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')
-- where id = 12 and modified_at = 1700176335
-- returning id as "id!", name as "name!", breed as "breed!", modified_at as "modified_at!"
