fn main() {
    // * to pass mutable reference, variable should also be mutable
    let mut s1 = String::from("hello");

    // * passing mutable reference
    let len = calc_len(&mut s1);

    println!("Length of {} is {}", s1, len);

    // * mutable references have only one restriction -
    //   you can have ONLY ONE MUTABLE REFERENCE
    let mut s2 = String::from("hello");

    // * curly braces can be used to create a new scope
    //   and it allows for multiple references,
    //   just not simultaneous ones
    {
        let r1 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference
      //  causing data race
    let r2 = &mut s2;

    // * we can't have mutable AND immutable reference at the same time,
    //   bc immutable reference means that we don't value to change

    // this code won't work
    let mut s3 = String::from("hello");

    // This gives error, bc scope of immutable reference
    // overlap with mutable one's
    // let r1 = &s3;
    // let r2 = &mut s3;
    // println!("{} {}", r1, r2);

    // This runs fine, bc scope immutable reference's scope is ended
    // before mutable one's
    let r1 = &s3;
    println!("{}", r1);
    // r1 scope is ended, since it's not used after this point

    let r2 = &mut s3;
    println!("{}", r2);

    // * dangling references are not a problem, since Rust won't let us
    // let reference_to_nothin = dangle();
}
// * this will give an error for above reason
// fn dangle() -> &String { // dangle returns reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // return a reference to String
// } // s goes out of scope, and is dropped. Its memory is deleted.

// * here we are not transfering ownership,
//   we are passing passing the reference
//   aka function is borrowing the value by reference
// * &Sttring - reference are by default immutable,
//   to modify a borrowed value we have to use
//   mutable reference
fn calc_len(s: &mut String) -> usize {
    s.push_str("workd");

    s.len()
}
