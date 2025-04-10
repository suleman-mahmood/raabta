-- Add migration script here

alter table invoice alter column attachment_id drop not null;
