use async_graphql::{Context, Object, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use crate::{
    models::{NewWeather, Weather},
    schema::weather_data,
    weather::fetch_weather,
};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn weather(&self, ctx: &Context<'_>, city: String, country_code: String) -> Result<Weather> {
        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        let api_key = std::env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set.");
        let weather_data = fetch_weather(&city, &country_code, &api_key).await?;

        let new_report = NewWeather {
            city: &weather_data.name,
            country_code: &country_code,
            description: &weather_data.weather[0].description,
            temp: weather_data.main.temp,
            humidity: weather_data.main.humidity,
            pressure: weather_data.main.pressure,
            wind_speed: weather_data.wind.speed,
        };

        let created_weather: Weather = diesel::insert_into(weather_data::table)
            .values(&new_report)
            .get_result(&mut conn)
            .await?;

        Ok(created_weather)
    }
}
