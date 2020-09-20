CREATE TABLE records (
    id SERIAL PRIMARY KEY,
    timer_id INTEGER NOT NULL REFERENCES timers(id),
    start_at TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    end_at TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    duration INTEGER NOT NULL CHECK(duration >= 0)
);
CREATE INDEX records_start_at_index ON records(start_at);
CREATE INDEX records_end_at_index ON records(end_at);
