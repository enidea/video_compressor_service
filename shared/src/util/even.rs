pub fn make_even_up(value: u32) -> u32 {
    if value % 2 == 0 {
        value
    } else {
        value + 1
    }
}
