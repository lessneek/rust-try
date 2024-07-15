#[test]
pub fn mute_some_xxx() {
    use std::sync::Mutex;

    let a = Mutex::new(42);

    let ax = *a.lock().unwrap();
    println!("{ax}");

    let ay = a.try_lock().unwrap();

    println!("{ay}");
}
