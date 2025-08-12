use utoipa::OpenApi;
use crate::schema::application::Application;

use crate::routes::application::*;
use crate::routes::order::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_application,
        get_application,
        update_application,
        delete_application,
        list_applications,
        insert_order,
        get_order,
        list_orders,
        update_order,
        delete_order

    ),
    components(schemas(Application))
)]

pub struct ApiDoc;