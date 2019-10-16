

type LineHandler<'a> = dyn Fn(&'a str) -> Option<&'a str>;

struct LineHandlerInfo<'a> {
  label: &'a str,
  match_literal: &'a str,
  f: LineHandler<'a>,
}

struct Game<'a> {
    handlers: Vec<Box<LineHandlerInfo<'a>>>,
}

impl Game<'_> {
  fn match_str<'a>(&'a mut self, label: &'a str, match_literal: &'a str, mut f: LineHandler) {
    let lh = Box::new(LineHandlerInfo {
      label,
      match_literal,
      f,
    });
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