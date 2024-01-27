-- Add up migration script here
create table todos (
    id INTEGER PRIMARY KEY ,
    title TEXT NOT NULL ,
    status INTEGER NOT NULL ,
    description TEXT ,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);