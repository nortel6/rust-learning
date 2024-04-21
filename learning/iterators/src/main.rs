fn main() {
    let v1 = vec![1, 2, 3];

    // There are also:
    // into_iter(), move ownership into iterator
    // iter_mut(), mutable references
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];

    // Remember iterators are lazy,
    // never runs until consumed
    // v1.iter().map(|x| x + 1);
    // collect() can consume the iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
