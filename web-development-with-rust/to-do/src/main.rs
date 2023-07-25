mod models;
mod config;
mod handlers; 

use crate::handlers::status;
use actix_web::{
    HttpServer,
    App,
    web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;


#[actix_rt::main]
async fn main() -> io::Result<()>{

    dotenv().ok();
    
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    // this is local dev, so connecting to the db with TLS isn't necessary

    println!("Starting the server at http://{}:{}", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        // the move keyword can only be used with closures
        // If any var (of outside scope) appears in the closure, it's automatically borrowed.
        // using the move keyword will ensure the closure actually owns them
        // in this case, the `App` will be created multiple time, so the `pool` must also be duplicated, borrowing the same var might lead to unexpected behaviors.
        App::new()
            .app_data(pool.clone()) // this var will be accessable in other actix functions
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
