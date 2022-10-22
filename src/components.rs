use maud::*;

pub struct Title(pub &'static str);

impl Render for Title {
  fn render(&self) -> Markup {
    html! {
      h1 ."text-xl" { (self.0) }
    }
  }
}

pub struct SubTitle(pub &'static str);

impl Render for SubTitle {
  fn render(&self) -> Markup {
    html! {
      h2 { (self.0) }
    }
  }
}

pub struct Text(pub &'static str);

impl Render for Text {
  fn render(&self) -> Markup {
    let contents = self.0.lines();

    html! {
      .text {
        @for content in contents {
          p { (content) }
        }
      }
    }
  }
}
