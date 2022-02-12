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

    // * Rust passes data strictly by value
    let mut x = 10;
    let y = x; // both variables are stored in stack

    x = 20;

    println!("x = {}", x);
    println!("y = {}", y);
}
