#[test]
fn should_send() {
    use std::sync::mpsc;
    use std::sync::mpsc::RecvError;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    assert_eq!(rx.recv().as_deref(), Ok("hi"));
    assert_eq!(rx.recv().as_deref(), Ok("from"));
    assert_eq!(rx.recv().as_deref(), Ok("the"));
    assert_eq!(rx.recv().as_deref(), Ok("thread"));
    assert_eq!(rx.recv(), Err(RecvError));
}
