struct Foo<'a> {
  label: &'a str,
  f: fn(&str) -> Option<&str>,
}

struct Baz<'a> {
    s: &'a str,
    v: Vec<Foo<'a>>
}

impl Baz<'_> {
  fn new<'a>(s: &'a str) -> Baz<'a> {
    Baz {
      s,
      v: Vec::new()
    }
  }
}

fn main() {
  let mut b = Baz::new("something");

  b.v.push(Foo {
    label: "some function",
    f: |x| Some(x),
  });

  (b.v[0].f)("hello");

}
