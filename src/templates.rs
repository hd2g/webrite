use maud::{DOCTYPE, Markup, html, Render};

const TITLE: &'static str = "Webrite";

fn footer(sitename: &'static str, link: &'static str) -> Markup {
  html! {
    footer .footer.sticky."bottom-0"."p-12"."bg-base-200".text-base-content {
      ."flow-1" {
        p .flow-row.justify-center { "Copyright (C) " a href=(link) { (sitename) } " All Right Reserved." }
      }
    }
  }
}

struct Drawer {
  content: Markup,
  side: Markup,
  footer: Markup,
}

impl Render for Drawer {
  fn render(&self) -> Markup {
    let Self {content, side, footer} = self;
    html! {
      .drawer {
        input #menu-button .drawer-toggle type="checkbox";
        .drawer-content {
          (content)
        }
        { (footer) }
        // TODO(#): implement y-full
        .drawer-side {
          label for="menu-button" .drawer-overlay;
          (side)
        }
      }
    }
  }
}

pub fn layout(body: Markup) -> Markup {
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
                  label for="menu-button" .btn.btn-square.btn-ghost {
                    svg .inline-block."w-6"."h-6".stroke-current xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {
                      path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6M16M4 12h16M4 18h16" {}
                    }
                  }
                  h1 ."text-4xl" {
                    a href="/" { (TITLE) }
                  }
                }
                (body);
              }
            },
            footer: (footer(TITLE, "/")),
            side: html! {
              ul .menu."p-4".overflow-y-auto.h-full."w-80"."bg-base-100" {
                li { a href="/" { "Home" } }
                li { a href="/news" { "News" } }
                li { a href="/contact" { "Contact" } }
                li { a href="/rss" { "Rss" } }
                li { a href="/github" { "GitHub" } }
              }
            },
          })
        }
      }
    }
  }
}
