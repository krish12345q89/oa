use actix_web::{web, HttpResponse, Responder};
use crate::{lmdb::{order::DBOrder, utils::DB}, schema::order::Order, scripts::utils::update_rows_for_order};

/// Insert a new Order
#[utoipa::path(
    post,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 201, description = "Order inserted successfully"),
        (status = 500, description = "Insert error")
    )
)]
pub async fn insert_order(db: web::Data<DB>, item: web::Json<Order>) -> impl Responder {
    match db.insert(item.into_inner()) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert error: {}", e)),
    }
}

/// Get single Order by id
#[utoipa::path(
    get,
    path = "/orders/{id}",
    params(
        ("id" = String, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order found", body = Order),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Get error")
    )
)]
pub async fn get_order(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    match db.get_single(path.into_inner()) {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => HttpResponse::NotFound().body("Order not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Get error: {}", e)),
    }
}

/// List all Orders
#[utoipa::path(
    get,
    path = "/orders",
    responses(
        (status = 200, description = "List all orders", body = [Order]),
        (status = 500, description = "List error")
    )
)]
pub async fn list_orders(db: web::Data<DB>) -> impl Responder {
    match db.get() {
        Ok(Some(orders)) => HttpResponse::Ok().json(orders),
        Ok(None) => HttpResponse::Ok().json(Vec::<Order>::new()), // empty list instead of None
        Err(e) => HttpResponse::InternalServerError().body(format!("List error: {}", e)),
    }
}

/// Update an existing Order
#[utoipa::path(
    put,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 200, description = "Order updated"),
        (status = 500, description = "Update error")
    )
)]
pub async fn update_order(db: web::Data<DB>, item: web::Json<Order>) -> impl Responder {
    let order = item.into_inner();

    // 1. Pehle DB me Order update kar
    if let Err(e) = db.put(order.clone()) {
        return HttpResponse::InternalServerError()
            .body(format!("DB update failed: {}", e));
    }

    // // 2. Order id se sheets ke existing rows fetch kar
    // // Assume order_id field unique id ke liye hai, agar tera alag id hai to use kar
    // let order_id = order.order_id.as_str();

    // let mut sheet1_row = match db.get_sheet1_row(order_id).await {
    //     Some(row) => row,
    //     None => vec!["".to_string(); 15],
    // };

    // let mut sheet2_row = match db.get_sheet2_row(order_id).await {
    //     Some(row) => row,
    //     None => vec!["".to_string(); 15],
    // };

    // // 3. Update rows based on order fields
    // update_rows_for_order(&order, &mut sheet1_row, &mut sheet2_row);

    // // 4. Save updated rows back
    // if let Err(e) = db.save_sheet1_row(order_id, sheet1_row).await {
    //     return HttpResponse::InternalServerError()
    //         .body(format!("Sheet1 update failed: {}", e));
    // }
    // if let Err(e) = db.save_sheet2_row(order_id, sheet2_row).await {
    //     return HttpResponse::InternalServerError()
    //         .body(format!("Sheet2 update failed: {}", e));
    // }

    HttpResponse::Ok().body("Order and sheets updated successfully")
}

/// Delete an Order by id
#[utoipa::path(
    delete,
    path = "/orders/{id}",
    params(
        ("id" = String, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order deleted"),
        (status = 500, description = "Delete error")
    )
)]
pub async fn delete_order(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    match db.delete(path.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Delete error: {}", e)),
    }
}

/// Configure routes for orders
pub fn order_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/orders")
            .route(web::post().to(insert_order))
            .route(web::put().to(update_order))
            .route(web::get().to(list_orders)),
    )
    .service(
        web::resource("/orders/{id}")
            .route(web::get().to(get_order))
            .route(web::delete().to(delete_order)),
    );
}
