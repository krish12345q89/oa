use actix_web::{ web, HttpResponse, Responder };
use crate::schema::application::Application;
use crate::lmdb::application::{ DBApplication, AppDatabase as DB };

// Handler to create an application
#[utoipa::path(
    post,
    path = "/applications",
    request_body = Application,
    responses(
        (status = 201, description = "Application created"),
        (status = 500, description = "Insert error")
    )
)]
pub async fn create_application(db: web::Data<DB>, item: web::Json<Application>) -> impl Responder {
    match db.save(&item.into_inner()) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert error: {}", e)),
    }
}

// Handler to get an application by id
#[utoipa::path(
    get,
    path = "/applications/{id}",
    params(
        ("id" = String, Path, description = "Application ID")
    ),
    responses(
        (status = 200, description = "Application found", body = Application),
        (status = 404, description = "Not found"),
        (status = 500, description = "Get error")
    )
)]
pub async fn get_application(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    match db.get(&path.into_inner()) {
        Ok(Some(app)) => HttpResponse::Ok().json(app),
        Ok(None) => HttpResponse::NotFound().body("Not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Get error: {}", e)),
    }
}

// Handler to update an application
#[utoipa::path(
    put,
    path = "/applications",
    request_body = Application,
    responses(
        (status = 200, description = "Application updated"),
        (status = 500, description = "Update error")
    )
)]
pub async fn update_application(db: web::Data<DB>, item: web::Json<Application>) -> impl Responder {
    match db.update(&item.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Update error: {}", e)),
    }
}

// Handler to delete an application by id
#[utoipa::path(
    delete,
    path = "/applications/{id}",
    params(
        ("id" = String, Path, description = "Application ID")
    ),
    responses(
        (status = 200, description = "Application deleted"),
        (status = 500, description = "Delete error")
    )
)]
pub async fn delete_application(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    match db.delete(&path.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Delete error: {}", e)),
    }
}

// Handler to list all applications (for demo, not efficient for large datasets)
#[utoipa::path(
    get,
    path = "/applications",
    responses(
        (status = 200, description = "List all applications", body = [Application]),
        (status = 500, description = "List error")
    )
)]
pub async fn list_applications(db: web::Data<DB>) -> impl Responder {
    match db.list() {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(e) => HttpResponse::InternalServerError().body(format!("List error: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::resource("/applications")
            .route(web::post().to(create_application))
            .route(web::put().to(update_application))
            .route(web::get().to(list_applications))
    ).service(
        web
            ::resource("/applications/{id}")
            .route(web::get().to(get_application))
            .route(web::delete().to(delete_application))
    );
}

        