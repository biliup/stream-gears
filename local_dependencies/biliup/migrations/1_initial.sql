create table if not exists live_streamers
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    url        TEXT                              not null UNIQUE default '',
    remark     TEXT                              not null        default '',
    filename   TEXT                              not null        default './video/%Y-%m-%d/%H_%M_%S{title}',
    split_time INTEGER,
    split_size INTEGER,
    upload_id  INTEGER
);

create table if not exists upload_streamers
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    template_name      TEXT                              NOT NULL UNIQUE,
    copyright          INTEGER                           not null default 1,
    source             TEXT                              not null default '',
    tid                INTEGER                           not null default 171,
    cover              TEXT                              not null default '',
    title              TEXT                              not null default '',
    'desc'             TEXT                              not null default '',
    dynamic            TEXT                              not null default '',
    tag                TEXT                              not null default '',
    dtime              INTEGER,
    interactive        INTEGER                           not null default 0,
    mission_id         INTEGER,
    dolby              INTEGER                           not null default 0,
    lossless_music     INTEGER                           not null default 0,
    no_reprint         INTEGER,
    up_selection_reply INTEGER                           not null default 0,
    up_close_reply     INTEGER                           not null default 0,
    up_close_danmu     INTEGER                           not null default 0,
    open_elec          INTEGER
);

-- alter table users
--     add constraint users_id_pk primary key (id);
--
-- create index if not exists users_email_idx on users (email);
insert or ignore into upload_streamers (template_name, copyright, source, cover, title, desc, dynamic, tag, dtime,
                                        interactive, mission_id, dolby, lossless_music, no_reprint, up_selection_reply,
                                        up_close_reply, up_close_danmu, open_elec)
values ('空模板', 1, '', '', '', '', '', '', null, 0, null, 0, 0, null, 0, 0, 0, null);