

//type LineHandler = fn(line: &str) -> Option<&str>;


#[derive(Debug)]
struct Game<'a> {
    handlers: Vec<LineHandlerInfo>,
}

impl Game {
  fn match_str<'a>(&'a mut self, label: &'a str, match_literal: &'a str, mut f: LineHandler) {
    let mut lh = LineHandlerInfo {
      label,
      match_literal,
      f,
    };
    self.handlers.push(lh);
  }

}

main() {
  let mut g = Game {
    handlers: Vec::new()
  }
  g.match_str("echo hello", "hello", |s| {
    println("{}", s);
  })
}