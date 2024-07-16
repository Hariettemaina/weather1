// @generated automatically by Diesel CLI.

diesel::table! {
    weather_data (id) {
        id -> Int4,
        city -> Varchar,
        country_code -> Varchar,
        description -> Varchar,
        temp -> Float8,
        humidity -> Float8,
        pressure -> Float8,
        wind_speed -> Float8,
    }
}
