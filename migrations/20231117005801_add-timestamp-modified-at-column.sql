-- Add migration script here
alter table mares
add column modified_at timestamp default 0 not null;

update mares
set modified_at = strftime('%s','now');
