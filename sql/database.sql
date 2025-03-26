create table "user" (
    "user_id" serial primary key,
    "user_name" text not null unique,
    "user_desc" text not null default '',
    "user_password" text not null default '',
    "user_email" text not null unique default '',
    "user_phone" text not null unique default '',
    "user_avatar_url" text not null default '',
    "user_level" int2 not null default 0,
    "user_status" int2 not null default 0,
    "user_identity" int2 not null default 0,
    "user_create_time" timestamp
    with
        time zone not null default now (),
        "user_update_time" timestamp
    with
        time zone not null default now ()
);
