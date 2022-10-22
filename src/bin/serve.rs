#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

use webrite::*;

fn main() {
  rocket::ignite()
    .mount("/", routes![index, contact])
    .mount("/static", StaticFiles::from("./static"))
    .launch();
}
