use webrite::*;

use std::{fs, path::Path};

use maud::Markup;

struct Target(fn() -> Markup, &'static str);

fn main() {
  let root_dir = Path::new("./static/html/");
  if root_dir.exists() {
    fs::remove_dir_all(root_dir.clone()).unwrap();
  }
  fs::create_dir(root_dir).unwrap();

  let targets = vec![
    Target(index, "index"),
    Target(contact, "contact")
  ];

  for Target(ep, filename) in targets {
    fs::write(
      root_dir.join(Path::new(format!("{}.html", filename).as_str())),
      ep().into_string()
    ).unwrap();
  }
}
