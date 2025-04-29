use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList};

fn collections() {
    // COLLECTIONS:

    /*
    The Rust standard library provides several powerful collection types
    in the std::collections module:
    - HashMap
    - HashSet
    - BTreeMap
    - BTreeSet
    - LinkedList
    - BinaryHeap

    These collections are usually stored on the heap and can dynamically
    grow or shrink during runtime.
    */

    // 1. HashMap:
    /*
    HashMap stores a mapping of unique keys to values.
    It does not guarantee any specific order of the keys.
    */
    let mut map = HashMap::new();
    map.insert("banana", 3);
    map.insert("apple", 5);
    map.insert("orange", 2);

    println!("HashMap: {:?}", map);
    // Example output: {"orange": 2, "banana": 3, "apple": 5}

    // Accessing a value safely
    if let Some(value) = map.get("apple") {
        println!("Value for 'apple': {}", value);
    }

    // 2. HashSet:
    /*
    HashSet stores unique elements without any particular order.
    */
    let mut set = HashSet::new();
    set.insert("blue");
    set.insert("red");
    set.insert("green");
    set.insert("blue"); // duplicate, will be ignored

    println!("HashSet: {:?}", set);
    // Example output: {"green", "blue", "red"}

    // 3. BTreeMap:
    /*
    BTreeMap stores key-value pairs sorted by key.
    */
    let mut btree_map = BTreeMap::new();
    btree_map.insert(10, "ten");
    btree_map.insert(5, "five");
    btree_map.insert(8, "eight");

    println!("BTreeMap: {:?}", btree_map);
    // Output: {5: "five", 8: "eight", 10: "ten"}

    // 4. BTreeSet:
    /*
    BTreeSet stores unique elements in sorted order.
    */
    let mut btree_set = BTreeSet::new();
    btree_set.insert(30);
    btree_set.insert(10);
    btree_set.insert(20);

    println!("BTreeSet: {:?}", btree_set);
    // Output: {10, 20, 30}

    // 5. LinkedList:
    /*
    LinkedList is a doubly-linked list.
    It allows efficient insertion and removal at the beginning and end.
    */
    let mut linked_list = LinkedList::new();
    linked_list.push_back(1);
    linked_list.push_back(2);
    linked_list.push_front(0);

    println!("LinkedList: {:?}", linked_list);
    // Output: [0, 1, 2]

    // 6. BinaryHeap:
    /*
    BinaryHeap is a max-heap by default.
    It always keeps the largest element at the top.
    */
    let mut heap = BinaryHeap::new();
    heap.push(4);
    heap.push(1);
    heap.push(7);
    heap.push(3);

    println!("BinaryHeap: {:?}", heap);
    // Example output: BinaryHeap { data: [7, 3, 4, 1] }

    // Peeking the largest element
    if let Some(max) = heap.peek() {
        println!("Max element in BinaryHeap: {}", max);
    }

    // COLLECTIONS:
    /*
    +--------------+--------------+-------------------------+
    | Collection   | Ordered?     | Allows Duplicates?       |
    +--------------+--------------+-------------------------+
    | HashMap      | No           | No (keys must be unique) |
    | HashSet      | No           | No                      |
    | BTreeMap     | Yes (by key) | No (keys must be unique) |
    | BTreeSet     | Yes          | No                      |
    | LinkedList   | Yes (sequence)| Yes                     |
    | BinaryHeap   | No (priority) | Yes                     |
    +--------------+--------------+-------------------------+

    Ordered?:
    - Yes: Elements are ordered (either by key or insertion).
    - No: Elements have no specific order (may vary at runtime).

    Allows Duplicates?:
    - No: Duplicate values (or keys) are not allowed.
    - Yes: Repeated values are allowed.

    Notes:
    - HashMap and BTreeMap store key-value pairs.
    - HashSet and BTreeSet store only values (no associated values).
    - LinkedList is good for frequent insertions/removals at ends.
    - BinaryHeap is a max-heap by default (biggest element at the top).

    Example:
    - HashMap<String, i32> maps a fruit name to a quantity.
    - HashSet<String> stores a list of unique colors.
    */
}
