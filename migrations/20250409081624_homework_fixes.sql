-- Add migration script here

alter table homework add column class_id bigint not null references class(id);

create table attachment (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  url text not null,
  thumbnail_url text not null,
  mime_type text not null,

  created_at timestamptz not null default now()
);

drop table homework_attachment;
create table homework_attachment (
  attachment_id bigint not null references attachment(id),
  homework_id bigint not null references homework(id),

	primary key(attachment_id, homework_id)
);
