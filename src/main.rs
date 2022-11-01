mod ddb_data;

use ddb_data::submission;

use rocket::{get, routes};
use rocket::response::status;

#[get("/")]
fn index() -> &'static str {
    "Hello, there!"
}

#[get("/doc")]
fn doc() -> &'static str {
    "Hello, doc!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index, doc])
        .launch()
        .await?;

    Ok(())
}