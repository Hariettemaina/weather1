use crate::graphql::QueryRoot;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};

use std::{env, io};

mod graphql;
mod models;
mod schema;
mod weather;

async fn graphql_handler(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Pool build error: {}", e)))?;

    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
                    .supports_credentials(),
            )
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(actix_web::guard::Post())
                    .to(graphql_handler),
            )
            .service(Files::new("/", "./my-weather-app").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
