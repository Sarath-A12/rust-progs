fn main() {
    //example of an iterator that does nothing

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // lazy iterator

    for val in v1_iter {
        println!("Got : {val}");
    }

    // Iterators implement the `Iterator` trait under the hood
    // Item , next(self) -> Optional<Item>

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // each call to next uses up the current `state?` of the
    // iterator

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    //v1_iter is invalid after this point on - because `sum` consumed it
    // v1_iter.next();
    assert_eq!(total, 6);

    // producing new iterators adapters
    // v1.iter().map(|x| x + 1);
    // since it's lazy we get warning message saying it's unused
    // to use it we will have the collect method
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    //iterators are 0 cost abstractions - meaning that the code essentially
    // compiles down to the lower level for loop mechanism thereby imposing
    // no penalty on performance
}
