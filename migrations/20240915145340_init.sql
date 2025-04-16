create type UserRole as enum (
	'STUDENT',
	'PARENT',
	'TEACHER',
	'SCHOOL_ADMIN'
);

CREATE TABLE raabta_user (
  id bigint primary key generated always as identity,
	public_id text not null unique,
	display_name text not null,
	email text not null unique,
	phone_number text,
	user_role UserRole not null,
	archived bool not null default false,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),

	parent_user_id bigint references raabta_user(id) default null
);

create table class (
  id bigint primary key generated always as identity,
	public_id text not null unique,
	display_name text not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);

create table user_class (
	class_id bigint not null references class(id),
	user_id bigint not null references raabta_user(id),

	PRIMARY KEY(class_id, user_id)
);

create table announcement (
  id bigint primary key generated always as identity,
	public_id text not null unique,
	content text not null,
	created_at timestamptz not null default now(),

	class_id bigint references class(id) default null,
	announcer_user_id bigint not null references raabta_user(id)
);

create table chat (
  id bigint primary key generated always as identity,
	public_id text not null unique,
	display_name text not null
);

create table chat_message(
  id bigint primary key generated always as identity,
	content text not null,
	created_at timestamptz not null default now(),

	chat_id bigint not null references chat(id),
	sender_user_id bigint not null references raabta_user(id)
);

create table chat_member(
	chat_id bigint not null references chat(id),
	member_user_id bigint not null references raabta_user(id),

	PRIMARY KEY(chat_id, member_user_id)
);

create table credentials (
	raabta_user_id bigint not null references raabta_user(id),
	plain_text_password text not null,

	PRIMARY KEY(raabta_user_id)
);

create unique index raabta_user_email_unique on raabta_user (email) where (not archived);
