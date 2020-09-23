CREATE TABLE tags_timers (
    tag_id INTEGER REFERENCES tags(id) NOT NULL,
    timer_id INTEGER REFERENCES timers(id) NOT NULL
);
CREATE INDEX tag_id_index ON tags_timers(tag_id);
CREATE INDEX timer_id_index ON tags_timers(timer_id);