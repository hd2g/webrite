#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_codegen;
#[macro_use] extern crate rocket_contrib;

use maud::{Render, Markup, html};

mod templates;
use templates::layout;

mod components;
use components::*;

#[get("/")]
pub fn index() -> Markup {
  layout(html! {
    #about {
      #about__brief {}
      #about__description {}
      #about__induce {}
    }
    #wiki-search {
      #wiki-search__title {}
      #wiki-search__search-box {}
    }
    #news {
      #news__title {}
      #news__feeds {}
    }
  })
}

struct ContactForm<'name, 'email, 'message> {
  name_label: &'name str,
  email_label: &'email str,
  message_label: &'message str,
}

impl Render for ContactForm<'static, 'static, 'static> {
  fn render(&self) -> Markup {
    html! {
      form .grid."grid-cols-1"."gap-6"."m-16" {
        label .block for="name" { (self.name_label) }
        input .form-input type="text";

        label .block for="emial" { (self.email_label) }
        input .form-input type="email";

        label .block for="message" { (self.message_label) }
        textarea .form-textarea {};

        label .block for="send" { "Send" }
        button type="button" {};
      }
    }
  }
}

#[get("/contact")]
pub fn contact() -> Markup {
  layout(html! {
    (Title("Contact"));
    (Text(""));
    (ContactForm {
      name_label: "Full name(or nickname)",
      email_label: "Email address",
      message_label: "Message",
    });
  })
}
