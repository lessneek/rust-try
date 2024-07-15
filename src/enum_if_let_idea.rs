#[derive(Debug)]
enum Foo {
    A,
    B(Bar),
    C(u8),
    D { x: u8, y: usize },
}

#[derive(Debug)]
struct Bar {
    i: usize,
    j: usize,
}

#[test]
fn example1() {
    let a = Foo::A;

    // Classic
    if let Foo::A = a {
        println!("Foo: {a:#?}")
    }

    #[cfg(if_let_idea)]
    foo.if_let(Foo::A).map(|f| println!("Foo: {f:#?}"));
}

#[test]
fn example2() {
    let b = Foo::B(Bar { i: 42, j: 142 });

    // Classic
    if let Foo::B(bar) = b {
        println!("Bar: {bar:#?}")
    }

    #[cfg(if_let_idea)]
    foo.if_let(Foo::B(bar) as bar)
        .map(|f| println!("Bar: {foo:#?}"));
}

#[allow(unused_macros)]
macro_rules! if_let {
    ($i:ident as $p:pat => $r:expr) => {
        if let $p = $i {
            Some($r)
        } else {
            None
        }
    };
}

#[test]
fn example3() {
    let d = Foo::D { x: 42, y: 142 };

    // Classic
    if let Foo::D { x, y } = d {
        println!("D {{ x: {x}, y: {y} }}");
    }

    if_let!(d as Foo::D { x, y } => (x, y)).map(|(x, y)| println!("D {{ x: {x}, y: {y} }}"));
}
