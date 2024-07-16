use async_graphql::{InputObject, SimpleObject};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};
use crate::schema::weather_data;


#[derive(InputObject, SimpleObject, Debug, Queryable, Selectable, Clone, AsChangeset)]
#[diesel(table_name = weather_data)]
pub struct Weather {
    pub id: i32,
    pub city: String,
    pub country_code: String,
    pub description: String,
    pub temp: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub wind_speed: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = weather_data)]
pub struct NewWeather<'a> {
    pub city: &'a str,
    pub country_code: &'a str,
    pub description: &'a str,
    pub temp: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub wind_speed: f64,
}
