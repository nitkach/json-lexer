-- Add migration script here
create table if not exists mares (
       id integer primary key autoincrement not null,
     name varchar not null,
    breed integer not null
);
