

// type LineHandler = fn(line: &str) -> Option<&str>;

struct LineHandlerInfo<'a> {
  label: &'a str,
  match_literal: &'a str,
  f: fn(&str) -> Option<&str>,
}

struct Game<'a> {
    handlers: Vec<LineHandlerInfo<'a>>,
}

impl Game<'_> {
  fn match_str<'a>(&'a mut self, label: &'a str, match_literal: &'a str, mut f: fn(&str) -> Option<&str>) {
    let mut lh = LineHandlerInfo {
      label,
      match_literal,
      f,
    };
    self.handlers.push(lh);
  }

}

fn main() {
  let mut g = Game {
    handlers: Vec::new()
  };
  g.match_str("echo hello", "hello", |s| {
    println!("{}", s);
    None
  });
}