use crate::functions::my_print;

mod functions;

fn main() {
   let mut x: u8 = 5;

    println!("{}", x);

    let x = x as i32 - 6;

    println!("{}", x);

    my_print(10);


}
