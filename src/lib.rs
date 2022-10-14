#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_codegen;
#[macro_use] extern crate rocket_contrib;

use maud::{Markup, html};

mod templates;

use templates::layout;

#[get("/")]
pub fn index() -> Markup {
  layout(html! {
    h1 { "ok" }
  })
}
