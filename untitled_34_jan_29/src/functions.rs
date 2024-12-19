

fn divisible_by_two(num: i32) -> bool {
    return num % 2 == 0;
}
fn my_print(num: i32) {
    println!("Is {} / by 2 := {}", num, divisible_by_two(num));

}

fn main() {
    my_print(10);
}