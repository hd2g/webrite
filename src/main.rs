#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_codegen;
#[macro_use] extern crate rocket_contrib;

use maud::{DOCTYPE, Markup, html, Render};
use rocket_contrib::serve::StaticFiles;

const TITLE: &'static str = "Webrite";

fn footer(sitename: &'static str, link: &'static str) -> Markup {
  html! {
    footer {
      "Copyright (C) "
      a href=(link) { (sitename) };
      " All Right Reserved."
    }
  }
}

struct Drawer {
  content: Markup,
  side: Markup,
}

impl Render for Drawer {
  fn render(&self) -> Markup {
    let Self {content, side} = self;
    html! {
      .drawer {
        input #menu-button .drawer-toggle type="checkbox";
        .drawer-content {
          (content)
        }
        .drawer-side {
          label for="menu-button" .drawer-overlay;
          (side)
        }
      }
    }
  }
}

fn layout(body: Markup) -> Markup {
  html! {
    (DOCTYPE)
    html {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width,initial-scale=1";
        link rel="stylesheet" type="text/css" href="/static/css/style.css";
        title { (TITLE) }
      }
      body {
        main {
          (Drawer {
            content: html! {
              .flex.flex-col {
                .w-full.navbar."bg-base-300" {
                  label for="menu-button" .btn.btn-primary.drawer-button {
                    "Open Drawer"
                  }
                }
                (body);
                (footer(TITLE, "/"));
              }
            },
            side: html! {
              ul .menu."p-4".overflow-y-auto."w-80"."bg-base-100" {
                li { a href="/" { "Home" }}
                li { a href="/about" { "About" }}
                li { a href="/contact" { "Contact" }}
                li { a href="/rss" { "Rss" }}
                li { a href="/github" { "GitHub" }}
              }
            },
          })
        }
      }
    }
  }
}


#[get("/")]
fn index() -> Markup {
  layout(html! {
    h1 { "ok" }
  })
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .mount("/static", StaticFiles::from("./static"))
    .launch();
}
