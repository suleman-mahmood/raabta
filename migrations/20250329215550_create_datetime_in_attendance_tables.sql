-- Add migration script here

alter table attendance add column marked_at timestamptz not null default now();
