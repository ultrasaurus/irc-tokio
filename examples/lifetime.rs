struct LineHandlerInfo<'a> {
  label: &'a str,
}

struct Game<'a> {
    handlers: Vec<LineHandlerInfo<'a>>,
}

impl<'b> Game<'b> {
  fn match_str(mut self, label: &'b str) {
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