use std::fs;
use heap_ds::heap::{Heap, HeapType};

#[test]
fn test_min_heap() {
    let buffer = fs::read_to_string("./input.txt").expect("Fatal: failed to read input"); 
    let input: Vec<&str> = buffer.split_ascii_whitespace().collect();
    let mut min_heap: Heap<i32> = Heap::new(HeapType::Min);

    for string in input {
        let num = str::parse::<i32>(string.trim())
            .expect("Failed to parse string");

        min_heap.insert(num);  // build a min heap from the input
    }

    assert_eq!(*min_heap.peek().unwrap(), -1);
    assert_eq!(min_heap.remove(), -1);
    assert_eq!(*min_heap.peek().unwrap(), 2);
}