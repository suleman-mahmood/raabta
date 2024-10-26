create type UserRole as enum (
	'STUDENT',
	'PARENT',
	'TEACHER',
	'SCHOOL_ADMIN'
);

CREATE TABLE raabta_user (
	id uuid PRIMARY KEY,
	public_id text not null unique,
	display_name text not null,
	email text not null unique,
	phone_number text,
	user_role UserRole not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),

	parent_user_id uuid references raabta_user(id) default null
);

create table class (
	id uuid PRIMARY KEY,
	display_name text not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);

create table user_class (
	class_id uuid not null references class(id),
	user_id uuid not null references raabta_user(id),

	PRIMARY KEY(class_id, user_id)
);

create table announcement (
	id uuid PRIMARY KEY,
	content text not null,
	created_at timestamptz not null default now(),

	class_id uuid references class(id) default null,
	announcer_user_id uuid not null references raabta_user(id)
);

create table chat (
	id uuid PRIMARY KEY,
	display_name text not null
);

create table chat_message(
	id uuid PRIMARY KEY,
	content text not null,
	created_at timestamptz not null default now(),

	chat_id uuid not null references chat(id),
	sender_user_id uuid not null references raabta_user(id)
);

create table chat_member(
	chat_id uuid not null references chat(id),
	member_user_id uuid not null references raabta_user(id),

	PRIMARY KEY(chat_id, member_user_id)
);

create table credentials (
	raabta_user_id uuid not null references raabta_user(id),
	plain_text_password text not null,

	PRIMARY KEY(raabta_user_id)
);
