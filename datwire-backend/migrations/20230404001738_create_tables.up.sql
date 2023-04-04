raise exception "THIS IS STILL WIP!";

create extension if not exists citext;

create table users (
  id uuid primary key default gen_random_uuid() not null,
  handle citext not null,
  handle_id integer default floor((random() * (9999)::double precision)) not null,
  creation_time timestamp with time zone default current_timestamp not null,
  last_online timestamp with time zone default current_timestamp not null,
  about text,
  email citext not null unique,
  password_hash text not null,
  unique (handle, handle_id)
  constraint check (((handle_id >= 0) AND (handle_id <= 9999)))
);

create table sessions (
  id uuid primary key default gen_random_uuid() not null,
  useragent text,
  token varchar(45) not null,
  address inet,
);
