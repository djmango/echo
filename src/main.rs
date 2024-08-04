use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_persist::PersistInstance;
use shuttle_runtime::SecretStore;
use config::AppConfig;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod config;
mod routes;
mod middleware;
mod models;
mod types;

#[derive(Clone)]
struct AppState {
    persist: PersistInstance,
    pool: PgPool,
    // memory_cache: Cache<String, HashMap<Uuid, Memory>>,
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/", api = routes::hello::ApiDoc),
    ),
    tags(
        (name = "echo", description = "Invisibiliy echo API, powering ghost and related services.")
    )
)]
struct ApiDoc;
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {    
    std::env::set_var("RUST_LOG", "actix_web=trace");
    let app_config = Arc::new(AppConfig::new(&secret_store).unwrap());
    let app_state = Arc::new(AppState {
        persist,
        pool: PgPool::connect(&app_config.db_connection_uri)
            .await
            .unwrap(),
    });

    let openapi = ApiDoc::openapi();

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(routes::hello::hello_world)
                .service(
                    web::scope("/devents")
                        .service(routes::devents::create_devent)
                        .service(routes::devents::get_devents_for_session)
                        .service(routes::devents::get_devents_for_recording)
                        .service(routes::devents::get_devent)
                )
                .service(
                    web::scope("/recordings")
                        .service(routes::recordings::fetch_save_url)
                )
                .service(
                    web::scope("/auth")
                        .service(routes::auth::login)
                        .service(routes::auth::signup)
                        .service(routes::auth::auth_callback)
                        .service(routes::auth::refresh_token)
                        .service(routes::auth::get_user)
                )
                .service(Scalar::with_url("/scalar", openapi))
                .wrap(middleware::auth::AuthenticationMiddleware {
                    app_config: app_config.clone(),
                })
                .wrap(middleware::logging::LoggingMiddleware)
                .wrap(Logger::new("%{r}a \"%r\" %s %b \"%{User-Agent}i\" %U %T"))
                .wrap(Cors::permissive())
                .app_data(web::Data::new(app_state.clone()))
                .app_data(web::Data::new(app_config.clone())),
        );
    };

    Ok(config.into())
}
