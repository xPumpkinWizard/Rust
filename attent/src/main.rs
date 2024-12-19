fn transform_array<F>(input: &[i32], transform: F) -> Vec<i32>
    where
        F: Fn(i32) ->i32,
{
    input.iter().map(|&x| transform(x)).collect()
}
fn main() {
    let double = |x| x * x;


    let input = [1, 2, 3, 4];
    let squared = transform_array(&input, double);
    let multiple = transform_array(&input, |x| x * 100);

    println!("{:?}", squared);
    println!("{:?}", multiple);
}