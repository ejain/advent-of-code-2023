use std::convert::From;
use std::path::PathBuf;

use memoize::memoize;

#[memoize]
pub fn do_something(_path1: PathBuf, _path2: PathBuf) -> String {
  // ...
  String::new()
}

fn main() {
    do_something(From::from("/a/b".to_string()), From::from("/c/d/".to_string()));
}
