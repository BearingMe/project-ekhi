use rocket::Error; // import structs
use rocket::{get, main, routes}; // import macros explicitly

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[main]
async fn main() -> Result<(), Error> {
    let _rocket = rocket::build() //
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
