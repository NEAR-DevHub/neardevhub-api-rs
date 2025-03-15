pub mod changelog;
pub mod db;
pub mod entrypoints;
pub mod nearblocks_client;
pub mod rpc_service;
pub mod types;

// Re-export commonly used items
pub use rpc_service::RpcService;
pub use types::PaginatedResponse;

use crate::entrypoints::ApiDoc;
use crate::rpc_service::Env;
use near_account_id::AccountId;
use rocket::{catch, catchers, get, routes};
use rocket_cors::{AllOrSome, AllowedOrigins, Origins};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Helper functions
pub fn separate_number_and_text(input: &str) -> (Option<i32>, String) {
    let mut number = None;
    let mut text = String::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() && !(number.is_some() && current_number.is_empty()) {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                number = current_number.parse().ok();
                current_number.clear();
            }
            text.push(c);
        }
    }

    if !current_number.is_empty() && number.is_none() {
        number = current_number.parse().ok();
    }

    (number, text.trim().to_string())
}

pub fn timestamp_to_date_string(timestamp_ns: i64) -> String {
    use chrono::{DateTime, Utc};

    let seconds = timestamp_ns / 1_000_000_000;
    let datetime: DateTime<Utc> = DateTime::from_timestamp(seconds, 0).unwrap_or_default();

    datetime.format("%Y-%m-%d").to_string()
}

#[get("/")]
fn index() -> &'static str {
    "Welcome from fly.io!!!!!"
}

// Allow robots to crawl the site
#[get("/robots.txt")]
fn robots() -> &'static str {
    "User-agent: *\nDisallow: /"
}

#[get("/")]
async fn test(contract: &rocket::State<AccountId>) -> String {
    format!("Welcome to {}", contract.inner())
}

#[catch(422)]
fn unprocessable_entity() -> &'static str {
    "Custom 422 Error: Unprocessable Entity"
}

#[catch(500)]
fn internal_server_error() -> &'static str {
    "Custom 500 Error: Internal Server Error"
}

#[catch(404)]
fn not_found() -> &'static str {
    "Custom 404 Error: Not Found"
}

#[catch(400)]
fn bad_request() -> &'static str {
    "Custom 400 Error: Bad Request"
}

pub fn rocket(rpc_service: Option<RpcService>) -> rocket::Rocket<rocket::Build> {
    dotenvy::dotenv().ok();

    let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");

    let exact_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://localhost:8080", // Playwright
        "http://127.0.0.1:8080", // Local development
        "https://dev.near.org",
        "https://near.social",
        "https://neardevhub.org",
        "https://devhub.near.page",
        "https://events-committee.near.page/",
        "https://infrastructure-committee.near.page/",
        "https://devhub-cache-api-rs.fly.dev",
        "https://infra-cache-api-rs.fly.dev",
        "https://events-cache-api-rs.fly.dev",
        // TODO Add prod urls here
    ]);
    let allowed_origins = Origins {
        allow_null: true, // Iframe simpleMDE mentioning proposals
        exact: exact_origins.unwrap().exact,
        ..Default::default()
    };

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllOrSome::Some(allowed_origins),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create cors config");

    let figment = rocket::Config::figment()
        .merge(rocket::Config::default())
        .merge(("databases.my_db.url", env.database_url));

    let contract: AccountId = env.contract.parse::<AccountId>().unwrap();
    let nearblocks_api_key = env.nearblocks_api_key;

    let rpc_service = rpc_service.unwrap_or_default();

    rocket::custom(figment)
        .attach(cors)
        .attach(db::stage())
        .mount("/", routes![robots, index])
        .manage(contract)
        .manage(nearblocks_api_key)
        .manage(rpc_service)
        .mount("/test", rocket::routes![test])
        .attach(entrypoints::stage())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .register(
            "/",
            catchers![
                unprocessable_entity,
                internal_server_error,
                not_found,
                bad_request
            ],
        )
}
