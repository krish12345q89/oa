use actix_web::{ web, App, HttpServer };
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod schema;
mod routes;
mod lmdb;
mod utopia;
mod scripts;

use crate::lmdb::application::AppDatabase;
use crate::lmdb::utils::{init_db, DB};
use crate::routes::application::config as application_config;
use crate::routes::order::order_config;
use crate::scripts::google_sheet_order::generate_access_token;
// use crate::scripts::google_sheet_order::start;
use crate::utopia::openapi::ApiDoc;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     // Initialize logging
//     env_logger::init();

//     let db_path = "./data/lmdb";
//     let db = AppDatabase::new(db_path)
//         .expect("Failed to initialize database");

//     info!("🚀 Starting server at http://localhost:8080");
//     info!("📚 API documentation available at http://localhost:8080/docs");
//     // start().await.expect("Failed to start Google Sheets script");
//     generate_access_token(&db).await.expect("Failed to generate access token");
//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(db.clone()))
//             .configure(application_config)
//             .service(
//                 SwaggerUi::new("/docs/{_:.*}")
//                     .url("/api-docs/openapi.json", ApiDoc::openapi())
//             )
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize your DB instance here (replace with your actual init)
    // Run async initialization before server starts
    generate_access_token().await.expect("Failed to generate access token");
    let db_path = "./data/lmdb";
    let db = init_db(db_path).await.expect("Failed to initialize database");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(application_config)
            .configure(order_config)  // your route config function
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
