pub mod db;
pub mod entrypoints;
pub mod nearblocks_client;
pub mod periodic_updater;
pub mod rpc_service;
pub mod types;

use chrono::DateTime;
use regex::Regex;

pub fn timestamp_to_date_string(timestamp: i64) -> String {
    // Convert the timestamp to a NaiveDateTime
    let datetime = DateTime::from_timestamp_nanos(timestamp);

    // Format the NaiveDateTime to a string in YYYY-MM-DD format
    datetime.format("%Y-%m-%d").to_string()
}

pub fn separate_number_and_text(s: &str) -> (Option<i32>, String) {
    let number_regex = Regex::new(r"\d+").unwrap();

    if let Some(matched) = number_regex.find(s) {
        let number_str = matched.as_str();
        let number = number_str.parse::<i32>().unwrap();
        let text = s.replacen(number_str, "", 1).trim().to_string();
        (Some(number), text)
    } else {
        (None, s.trim().to_string())
    }
}

use crate::entrypoints::ApiDoc;
use near_account_id::AccountId;
use rocket::{catch, catchers, get, launch, routes, State};
use rocket_cors::{AllOrSome, AllowedOrigins, Origins};
use types::Contract;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[cfg(test)]
mod tests;

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
async fn test(contract: &State<AccountId>) -> String {
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

#[derive(Debug, serde::Deserialize)]
pub struct Env {
    contract: Contract,
    database_url: String,
    nearblocks_api_key: String,
    #[serde(default)]
    proposal_sync_interval: u64,
    #[serde(default)]
    rfp_sync_interval: u64,
}

#[launch]
fn rocket() -> _ {
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
    let proposal_sync_interval = env.proposal_sync_interval;
    let rfp_sync_interval = env.rfp_sync_interval;

    rocket::custom(figment)
        .attach(cors)
        .attach(db::stage())
        .mount("/", routes![robots, index])
        .manage(contract)
        .manage(nearblocks_api_key)
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
        .attach(periodic_updater::stage(
            proposal_sync_interval,
            rfp_sync_interval,
        ))
}
