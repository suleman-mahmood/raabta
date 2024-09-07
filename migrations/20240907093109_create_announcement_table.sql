-- Add migration script here

create table announcement(
	id uuid primary key,
	name text not null,
	announcement text not null,
	create_datetime timestamptz not null default current_timestamp
)
