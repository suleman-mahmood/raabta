-- Add migration script here

create table homework (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  title text not null,
  prompt text not null,

	teacher_user_id uuid not null references raabta_user(id),

  deadline timestamptz not null,
  created_at timestamptz not null default now()
);

create table homework_attachment (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  url text not null,
  thumbnail_url text not null,
  mime_type text not null,
  homework_id bigint not null references homework(id),

  created_at timestamptz not null default now()
);
