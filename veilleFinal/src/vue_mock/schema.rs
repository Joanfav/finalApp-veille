use diesel::table;

table! {
    images (id) {
        id -> Int4,
        filepath -> Varchar,
        file_content -> Bytea,
        rotation -> Int4,
        brightness -> Int4,
        crop_x -> Nullable<Int4>,
        crop_y -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}
