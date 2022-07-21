// Functions are exported by default;
fn factorial(num) {
    if num <= 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}
