pub fn convert((t, u): (i32, char)) -> i32 {
    return match u {
        'C' => (t * 9 / 5) + 32,
        'F' => (t - 32) * 9 / 5,
        _ => 0,
    };
}
