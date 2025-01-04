use seq_set::SeqSet;

/// Test struct
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_insert_and_contains() {
    let mut set = SeqSet::new();
    assert!(set.insert(1));
    assert!(set.insert(2));
    assert!(set.insert(3));
    assert!(!set.insert(2)); // Intentional duplicate insertion

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));

    assert_eq!(set.len(), 3);
    assert!(!set.is_empty());
}

#[test]
fn test_remove() {
    let mut set = SeqSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("cherry");

    assert!(set.remove(&"banana"));
    assert!(!set.contains(&"banana"));
    assert_eq!(set.len(), 2);
    assert!(!set.is_empty());

    // this should fail, since it is not in the set
    assert!(!set.remove(&"orange"));
    assert_eq!(set.len(), 2);
}

#[test]
fn test_iteration_order() {
    let mut set = SeqSet::new();
    set.insert("first");
    set.insert("second");
    set.insert("third");

    let elements: Vec<&str> = set.iter().cloned().collect();
    assert_eq!(elements, vec!["first", "second", "third"]);
}

#[test]
fn test_remove_and_iterate() {
    let mut set = SeqSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("cherry");
    set.insert("orange");

    assert!(set.remove(&"banana"));

    let elements: Vec<&str> = set.iter().cloned().collect();
    assert_eq!(elements, vec!["apple", "cherry", "orange"]);
}

#[test]
fn test_clear() {
    let mut set = SeqSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    assert_eq!(set.len(), 3);
    set.clear();
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());

    assert!(!set.contains(&10));
    assert!(!set.contains(&20));
    assert!(!set.contains(&30));
}

#[test]
fn test_with_custom_struct() {
    let mut set = SeqSet::new();

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = Point { x: 5, y: 6 };

    assert!(set.insert(p1.clone()));
    assert!(set.insert(p2.clone()));
    assert!(set.insert(p3.clone()));
    assert!(!set.insert(p2.clone())); // Duplicate insertion

    assert!(set.contains(&p1));
    assert!(set.contains(&p2));
    assert!(set.contains(&p3));

    let elements: Vec<Point> = set.iter().cloned().collect();
    assert_eq!(elements, vec![p1, p2, p3]);

    assert!(set.remove(&p2));

    let elements: Vec<Point> = set.iter().cloned().collect();
    assert_eq!(elements, vec![p1, p3]);
}

#[test]
fn test_iterate_empty_set() {
    let set: SeqSet<i32> = SeqSet::new();
    let elements: Vec<&i32> = set.iter().collect();
    assert!(elements.is_empty());
}

#[test]
fn test_multiple_insertions_and_removals() {
    let mut set = SeqSet::new();

    for i in 1..=5 {
        assert!(set.insert(i));
    }

    assert!(set.remove(&2));
    assert!(set.remove(&4));

    assert!(set.insert(6));
    assert!(set.insert(7));

    let elements: Vec<i32> = set.iter().cloned().collect();
    assert_eq!(elements, vec![1, 3, 5, 6, 7]);

    assert!(!set.contains(&2));
    assert!(!set.contains(&4));
}

#[test]
fn test_insert_remove_insert() {
    let mut set = SeqSet::new();

    assert!(set.insert("alpha"));
    assert!(set.insert("beta"));
    assert!(set.insert("gamma"));

    assert!(set.remove(&"beta"));

    // Insert the removed element again
    assert!(set.insert("beta"));

    let elements: Vec<_> = set.iter().cloned().collect();
    assert_eq!(elements, vec!["alpha", "gamma", "beta"]);
}

#[test]
fn test_into_iter_consuming() {
    let mut set = SeqSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let mut iter = set.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_references() {
    let mut set = SeqSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("cherry");

    let elements: Vec<&str> = (&set).into_iter().cloned().collect();
    assert_eq!(elements, vec!["apple", "banana", "cherry"]);

    // Ensure the set is still intact
    assert!(set.contains(&"apple"));
    assert!(set.contains(&"banana"));
    assert!(set.contains(&"cherry"));
}

#[test]
fn test_into_iter_mutable_references() {
    let mut set = SeqSet::new();
    set.insert(11);
    set.insert(12);
    set.insert(13);

    for num in &mut set {
        *num += 1;
    }

    let elements: Vec<i32> = set.iter().cloned().collect();
    assert_eq!(elements, vec![12, 13, 14]);
}
