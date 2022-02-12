fn main() {
    // * use 'mut' to reassign another value
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // * always immutable, can't use shadowing
    // * may be set only to a constant expr.,
    // not the result of a func call/val computed at runtime
    const Y: u32 = 123_001;
    println!("{}", Y);

    // * shadowing, first spaces is shadowed by second spaces
    // * now we don't have to come up with diff names
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of y is: {}", spaces);

    // * but we're not allowed to mutate type:
    // let mut spaces = "   ";
    // spaces = spaces.len(); // expected `&str`, found `usize`
}
