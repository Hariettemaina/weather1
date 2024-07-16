-- Your SQL goes here
CREATE TABLE IF NOT EXISTS weather_data (
    id SERIAL PRIMARY KEY,
    city VARCHAR NOT NULL,
    country_code VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    temp FLOAT NOT NULL,
    humidity FLOAT NOT NULL,
    pressure FLOAT NOT NULL,
    wind_speed FLOAT NOT NULL
);
