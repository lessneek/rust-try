struct Foo {
    value: Vec<usize>,
}

impl Foo {
    pub fn new(size: usize) -> Self {
        Self {
            value: (0..size).collect()
        }
    }

    pub fn value(&self) -> &[usize] {
        self.value.as_slice()
    }
}

// impl<'a> Iterator for Foo<'a> {
//     type Item = &'a [usize];

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.value())
//     }
// }

// #[test]
// fn test_foo_iterator() {
//     let foo = Foo::new(42);

//     for v in foo {
//         println!("{v:?}")
//     }
// }
