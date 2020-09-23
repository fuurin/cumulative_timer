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

allow_tables_to_appear_in_same_query!(
    records,
    tags,
    timers,
    users,
);
