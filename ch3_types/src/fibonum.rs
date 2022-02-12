pub fn generate(n: i32) -> i32 {
    return match n {
        1 | 2 => 1,
        _ => generate(n - 1) + generate(n - 2),
    };
}
