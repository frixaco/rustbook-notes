fn main() {
    // * Stack - last in, first out; push onto stack/popping off the stack;
    //   data must have known, fixed size
    // * Heap  - data with unknown size; allocating(on the heap);
    //   returns pointer after putting data finds an empty spot
    // * Pushing to stack is faster than allocating on the heap (time to find space)
    // * Accessing data in the heap is slower than on stack
    // * Ownerships is to manage heap data

    // * string literal are stored in stack and are immutable - pushed onto stack at runtime,
    //   String type is mutable and stored in heap - space is allocated in runtime

    // let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // * When a variable goes out of scope 'drop' function is automatically called to
    //   deallocate memory from heap

    // * Rust passes data strictly by value because
    // * types that have known size at compile time will be stored entirely store
    //   on stack, so copies are quick to make => no difference shallow and deep copy
    //   for these types
    let mut x = 10;
    let y = x; // both variables are stored in stack, because they have known, fixed size

    x = 20;

    println!("x = {}", x);
    println!("y = {}", y);

    // * However, it's not same for e.g. Strings:
    //   String's length, capacity and pointer to the contents
    //   are stored on stack. And the pointer points to the contents of the string
    //   on heap.

    // * Code below will create s2 with the copy of the data on stack (pointer, length, capacity),
    //   but the content on the heap won't be copied (to improve perfomances).

    // * Also, s1 will be invalidated, so Rust won't have to free memory for both variables, only s2
    //   That's shallow copy - move, in Rust. So no deep copies => improve perfomance
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // If we want to copy both the stack and heap data we can use method clone
    let s1 = String::from("world");
    let s2 = s1.clone();

    // * Passing a variable to function will move or copy the variable like assignment.
    let s3 = String::from("hello"); // s3 comes into scope

    // * s3's value moves into the function and so is no longer valid here
    takes_ownership(s3);

    // * Using s3 now will throw compile time error

    let x1 = 5; // x comes into scope

    // * x1 would move into the function,
    // but i32 is Copy,
    makes_copy(x1);

    // so it's okay to still use x1 afterward

    // * s4 comes into scope
    let s4 = String::from("hello");

    // * s4 is moved into takes_and_gives_back, which also moves its return value into s5
    // * Ownership goes from s4 to function, and then to s5
    let s5 = takes_and_gives_back(s4);

    // * Transfering ownership and taking it back is tedious,
    // so intead we can use - references
} // * s5 is dropped, s4 was moved so nothing happens

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String {
    // * a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
