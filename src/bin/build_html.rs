use webrite::*;

use std::{fs, path::Path};

fn main() {
  let root_dir = Path::new("./static/html/");
  if root_dir.exists() {
    fs::remove_dir_all(root_dir.clone()).unwrap();
  }
  fs::create_dir(root_dir).unwrap();

  let output_path = root_dir.join(Path::new("index.html"));

  let contents = index().into_string();
  fs::write(output_path, contents).unwrap();
}
