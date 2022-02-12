mod farenheit_to_celcius;
mod fibonum;

fn main() {
    // * Scalar types are 4: integers, floating-point numbers
    //                      Booleans, characters
    // * two's complement:
    // - left bit is for sign +-
    // - to get negative you flip the bits (0->1) and add 1
    // So for signed 4 bit integer stores -(2^3)=-8 (1000)
    // to 2^3-1=7 (0111)
    // where first bit is 1s, second 2s, third 4s and
    // left one can be thought of as -8:
    //      1000 - -8
    //      1001 - -7
    //      ....
    //      1111 - -1
    //      0000 - 0
    //      0110 - 6
    // *  when integer overflow occurs, Rust performs
    // two's complement wrapping

    // * f64 is default, more precise than f32 and
    // roughly has same speed as f32
    let remainder = 10.1 % 5.2;
    println!("Remainder is {}", remainder);

    // * Compound types: tuples and arrays
    let tup: (i32, f64, char) = (500, 123.456, 'a');
    let (_first, second, _) = tup;
    println!("The first value is: {}", tup.0);
    println!("The second value is: {}", second);

    // * Arrays:
    //      - fixed size
    //      - stored on the stack
    let arr: [i32; 3] = [1, -2, 3];
    println!("{}", arr[0]);
    let _arr = [0; 5]; // [0, 0, 0, 0, 0]

    let _x = 5;
    // * { } is an expression that evaluates to 4 = 3 + 1
    // * expressions don't include semicolons;
    // if included they become statements (return nothing)
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let fv = plus_five(10);
    println!("The value of fv is: {}", fv);

    // * if is an expression
    let condition = true;
    // arms of if statement should return same type
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // * 'break <expression>' inside loop returns values
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter / 2;
        }
    };
    println!("The value of result is: {}", result);

    let a = [1, 2, 3, 4, 5];
    // * .iter() for arrays
    for _num in a.iter() {
        // * iterate through range: (1..10)
        for _i in (1..100).rev() {}
    }

    // ==================================================

    println!("{}", farenheit_to_celcius::convert((32, 'F')));

    println!("{}", fibonum::generate(6));
}

fn plus_five(x: i32) -> i32 {
    // no semicolon
    x + 5
}
