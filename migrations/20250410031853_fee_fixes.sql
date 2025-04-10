-- Add migration script here

create table fee_payer (
  id bigint primary key generated always as identity,
  fee_id bigint not null references fee(id),
  payer_user_id uuid not null references raabta_user(id),

  unique(fee_id, payer_user_id)
);

alter table invoice add column fee_payer_id bigint not null references fee_payer(id);
alter table invoice add column attachment_id bigint not null references attachment(id);

create type InvoicePaymentStatus as enum (
  'PENDING',
  'PAID',
  'EXPIRED'
);

alter table invoice add column payment_status InvoicePaymentStatus not null default 'PENDING'::InvoicePaymentStatus;
alter table invoice alter column payment_method drop not null;

alter table invoice
  alter column paid_date drop not null,
  alter column paid_date drop default;

alter table invoice add column created_at timestamptz not null default now();

alter table invoice drop column fee_id;
alter table invoice drop column payer_user_id;
