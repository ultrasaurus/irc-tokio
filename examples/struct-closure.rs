struct Foo {
  f: fn(&str) -> Option<&str>,
}

struct Baz {
    v: Vec<Foo>
}

fn main() {
  let mut b = Baz {
    v: Vec::new()
  };

  b.v.push(Foo {
    f: |x| Some(x),
  });

  (b.v[0].f)("hello");

}
