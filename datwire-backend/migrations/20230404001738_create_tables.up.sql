CREATE COLLATION case_insensitive (
  provider = icu,
  locale = 'und-u-ks-level2',
  deterministic = false
);

CREATE TABLE user_uploads (
  id   UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  mime TEXT COLLATE "case_insensitive",
  name TEXT,
  hash TEXT,
  data BYTEA
);

CREATE TABLE users (
  id              UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  handle          TEXT NOT NULL COLLATE "case_insensitive",
  tag             INTEGER NOT NULL DEFAULT floor(random() * 9999) CHECK (tag BETWEEN 0 AND 9999),
  password_hash   TEXT NOT NULL,
  creation_time   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  last_online     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  about           TEXT,
  email           TEXT NOT NULL COLLATE "case_insensitive" UNIQUE,
  profile_picture UUID REFERENCES user_uploads (id) ON DELETE SET NULL,
  banner          UUID REFERENCES user_uploads (id) ON DELETE SET NULL,
  CONSTRAINT unique_handle_tag UNIQUE (handle, tag)
);
CREATE INDEX users_handle ON users (handle);

CREATE TABLE sessions (
  id         UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  token      TEXT NOT NULL UNIQUE,
  useragent  TEXT,
  ip_address INET,
  last_used  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE guilds (
  id          UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  icon        UUID REFERENCES user_uploads (id) ON DELETE SET NULL,
  banner      UUID REFERENCES user_uploads (id) ON DELETE SET NULL,
  name        TEXT NOT NULL,
  owner       UUID REFERENCES users (id) ON DELETE SET NULL,
  description TEXT
);

CREATE TABLE users_guilds (
  guilds_id    UUID REFERENCES guilds (id) ON DELETE CASCADE,
  user_id      UUID REFERENCES users (id) ON DELETE CASCADE,
  join_time    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  display_name TEXT,
  CONSTRAINT users_guilds_pkey PRIMARY KEY (guilds_id, user_id)
);

CREATE TABLE channels (
  id       UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  name     TEXT NOT NULL COLLATE "case_insensitive",
  guild_id UUID NOT NULL REFERENCES guilds (id) ON DELETE CASCADE,
  CONSTRAINT unique_guild_channel_name UNIQUE (guild_id, name)
);

CREATE TABLE messages (
  id       UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  channel  UUID NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
  user_id  UUID REFERENCES users (id) ON DELETE SET NULL,
  reply_to UUID REFERENCES messages (id) ON DELETE SET NULL CHECK (reply_to <> id),
  body     TEXT NOT NULL,
  sent_on  TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE message_attachments (
  attachment_id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  message_id    UUID NOT NULL REFERENCES messages (id) ON DELETE CASCADE,
  attachment    UUID NOT NULL REFERENCES user_uploads (id) ON DELETE CASCADE
);
CREATE INDEX message_id_attachments_index ON message_attachments (message_id);
