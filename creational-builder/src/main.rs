#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        Self {
            bar: String::from("x"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;

        self
    }

    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}

fn main() {
    let foo = Foo {
        bar: String::from("y"),
    };

    let foo_from_builder = FooBuilder::new().name(String::from("y")).build();

    assert_eq!(foo, foo_from_builder);
}
