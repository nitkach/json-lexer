-- SQLite
-- create table mares (
--        id integer primary key autoincrement,
--      name varchar,
--     breed integer
-- );

-- create table test (
--      name varchar,
--     breed integer
-- );

-- 0 - unicorn, 1 - earth, 2 - pegasus



-- ("Twilight Sparkle", 0),
-- ("Applejack", 1);

-- insert into mares (name, breed)
-- values ("no mare", null);

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

-- select *
-- from mares;

update mares
set breed = 1, name = '2 EDIT'
where id = 44
returning *
