-- Add migration script here

create type Recurrence as enum (
  'ONCE',
  'WEEKLY',
  'FORTNIGHTLY',
  'MONTHLY',
  'QUATERLY',
  'HALF_YEARLY',
  'YEARLY'
);

create table fee (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  name text not null,
  description text,
  recurrence Recurrence not null,
  recurring_cycles_count int not null,

  invoice_date timestamptz not null,
  due_date timestamptz not null,
  created_at timestamptz not null default now()
);

create table fee_line_item (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  fee_id bigint not null references fee(id),

  name text not null,
  amount int not null,
  discount_percentage int not null
);

create type InvoicePaymentMethod as enum (
  'CASH',
  'BANK_TRANSFER',
  'PAYMENT_GATEWAY'
);

create table invoice (
  id bigint primary key generated always as identity,
  public_id text not null unique,

  fee_id bigint not null references fee(id),
  payer_user_id uuid not null references raabta_user(id),
  payment_method InvoicePaymentMethod not null,

  paid_date timestamptz not null default now()
);
