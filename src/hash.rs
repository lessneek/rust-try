use std::collections::BTreeSet;

pub fn blablabla_hash() {
    let mut set = BTreeSet::new();

    set.insert(vec![1, 2, 3]);
    set.insert(vec![3, 2, 1]);

    for item in set {
        println!("{item:?}");
    }
}

#[test]
fn test_hashset() {
    let mut set = BTreeSet::new();

    set.insert(vec![1, 2, 3]);
    set.insert(vec![3, 2, 1]);

    for item in set {
        println!("{item:?}");
    }
}
