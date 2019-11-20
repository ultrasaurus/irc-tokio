struct Foo<'a> {
  Bar: &'a str,
}

struct Baz<'a> {
    Baz: Vec<Foo<'a>>
}

fn main() {}
