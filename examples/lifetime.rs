struct LineHandlerInfo<'a> {
  label: &'a str,
}

struct Game<'a> {
    handlers: Vec<LineHandlerInfo<'a>>,
}

impl Game<'_> {
  fn match_str<'a>(&'a mut self, label: &'a str) {
    let mut lh = LineHandlerInfo {label};
    self.handlers.push(lh);
  }

}

fn main() {
  let mut g = Game {
    handlers: Vec::new()
  };
  g.match_str("echo hello");
}