table! {
    day_list (id) {
        id -> Integer,
        day_id -> Integer,
        list_id -> Integer,
        norder -> Integer,
    }
}

table! {
    days (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    lists (id) {
        id -> Integer,
        youtube_id -> Text,
        name -> Text,
        reverse -> Bool,
    }
}

table! {
    videos (id) {
        id -> Integer,
        list_id -> Integer,
        youtube_id -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    day_list,
    days,
    lists,
    videos,
);
