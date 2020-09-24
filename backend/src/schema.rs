table! {
    records (id) {
        id -> Int4,
        timer_id -> Int4,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
        duration -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        uid -> Varchar,
        name -> Varchar,
    }
}

table! {
    tags_timers (tag_id, timer_id) {
        tag_id -> Int4,
        timer_id -> Int4,
    }
}

table! {
    timers (id) {
        id -> Int4,
        uid -> Varchar,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

joinable!(records -> timers (timer_id));
joinable!(tags_timers -> tags (tag_id));
joinable!(tags_timers -> timers (timer_id));

allow_tables_to_appear_in_same_query!(
    records,
    tags,
    tags_timers,
    timers,
    users,
);
