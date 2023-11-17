-- Add migration script here
alter table test_mares
add column modified_at datetime not null default 0;

update test_mares
set modified_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW');
