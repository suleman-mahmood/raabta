-- Add migration script here

create type AttendanceMethod as enum (
  'MANUAL',
  'NFC_CARD_SCAN'
);

create type AttendanceType as enum (
  'ENTRY',
  'EXIT',
  'CLASS_ENTRY'
);

create type AttendanceLocation as enum (
  'MAIN_GATE',
  'CLASS'
);

create table attendance (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  attendance_method AttendanceMethod not null,
  attendance_type AttendanceType not null,
  attendance_location AttendanceLocation not null,

	attendee_user_id bigint not null references raabta_user(id),
	marker_user_id bigint references raabta_user(id) default null
);
