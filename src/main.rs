#[macro_use]
extern crate rocket;

use rocket_cors::CorsOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod api;
mod database;
mod models;
mod utils;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .connect(&*database_url.unwrap())
        .await?;
    let cors = CorsOptions::default().to_cors().unwrap();

    rocket::build()
        .mount(
            "/api/auth",
            routes![
                api::authentications::login,
                api::authentications::logout,
                api::authentications::register
            ],
        )
        .mount(
            "/api/stores",
            routes![
                api::stores::stores::get_store,
                api::stores::stores::create_store,
                api::stores::stores::update_store,
                api::stores::stores::delete_store,
                api::stores::users::get_store_users,
                api::stores::users::get_store_user,
                api::stores::users::create_store_user,
                api::stores::users::update_store_user,
                api::stores::users::delete_store_user,
                api::stores::roles::get_store_roles,
                api::stores::roles::get_store_role,
                api::stores::roles::create_store_role,
                api::stores::roles::update_store_role,
                api::stores::roles::delete_store_role,
                api::stores::role_users::get_store_role_users,
                api::stores::role_users::create_store_role_user,
                api::stores::role_users::delete_store_role_user,
                api::stores::user_roles::get_store_user_roles,
                api::stores::user_roles::create_store_user_role,
                api::stores::user_roles::delete_store_user_role,
            ],
        )
        .manage(pool)
        .attach(cors)
        .launch()
        .await?;
    Ok(())
}
