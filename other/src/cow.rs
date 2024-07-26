#[test]
pub fn test_cow() {
    use std::borrow::Cow;

    let mut a = "Hello ".to_string();

    let mut x = Cow::from(&a);
    println!("{x}");
    
    x.to_mut().push_str("HIHIHI!");
    
    println!("{x}");
    // drop(x);

    a.push_str("WORLD!");

    println!("{a}");
}
