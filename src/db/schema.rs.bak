table! {
    day_list (id) {
        id -> Nullable<Integer>,
        day_id -> Integer,
        list_id -> Integer,
        norder -> Integer,
    }
}

table! {
    days (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    lists (id) {
        id -> Nullable<Integer>,
        youtube_id -> Text,
        name -> Text,
        reverse -> Nullable<Integer>,
    }
}

table! {
    videos (id) {
        id -> Nullable<Integer>,
        list_id -> Integer,
        youtube_id -> Text,
        published -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    day_list,
    days,
    lists,
    videos,
);
