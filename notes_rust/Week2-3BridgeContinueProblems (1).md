#So in what way in practical terms (Ownership and Borrowing) affect our programming
When we move to concurrent programming, it will have a new meaning, but smart pointers will do most hard lifting,for know we need to understand how to use them in procedural programming.


#A general guideline for working with references and functions is to prefer using an immutable reference whenever possible. This will ensure that the function only accesses the data and does not take ownership of it, making the code simpler and reducing the risk of unintended changes to the data. If ownership transfer is not required, using a reference is the most straightforward solution.


#In systems and network programming, also in any kind of distributed systems , it's much better idea to think about data in terms of bytes.
Rust go further and incorporates this idea in everything 
From now on everything is a byte :)

Google interview story 2 years ago:

Problem: You have a text file read last 10 lines.

I did not ask how many bytes one letter will take, before I reach separator like \n line character 

1 letter can take 1,2,3,4 bytes per character

Follow up: What you gonna do if you need to parse 1000 files and they are distributed across many servers

Welcome to the world of programming.

# Code Section:


fn data_lives_inside_stack() {
    fn get_num(num: i32) {
        println!("{}",num);
    }

    let my_num:i32 = 10;
    get_num(my_num);
    // I can reuse same variable, no need to worry about ownership rule
    
    get_num(my_num);
    
}

fn data_lives_inside_heap() {
    // When you create a data, which wil be located in heap, you get back only pointer, and now you need to be very careful

    fn print_string(word: String) {
        println!("{}",word);
    }

    let msg = String::from("Hello");
    print_string(msg);
    //print_string(msg); // msg contains a pointer to the data located in heap, now you have to be careful, cause I need to worry about ownership rules
    
}

fn naive_solution_just_clone() {
    
    fn print_string(word: String) {
        println!("{}",word);
    }

    let msg = String::from("Hello");
    print_string(msg.clone());
    print_string(msg);
}

fn cost_for_naive_solution() {
    // with a bad code you usually pay a huge price
    fn get_length(input: String) {
        println!("It's {} words long.", input.split_whitespace().count());
    }

    use std::time::Instant;

    let start = Instant::now();

    let mut my_string = String::new();
    for _ in 0..10{
        my_string.push_str("Here are some more words "); // push the words on
        get_length(my_string.clone()); // gives it a clone every time
    }

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.5} seconds", elapsed.as_secs_f64());
    //Elapsed time: 1.9049 seconds for 1000 iteration
}

fn use_reference_smart_decision() {
    fn get_length(input: &String) {
        println!("It's {} words long.", input.split_whitespace().count());
    }

    use std::time::Instant;

    let start = Instant::now();

    let mut my_string = String::new();
    for _ in 0..10 {
        my_string.push_str("Here are some more words ");
        get_length(&my_string); // 
    }

    let elapsed = start.elapsed();
    println!("Elapsed time: {:.5} seconds", elapsed.as_secs_f64());
    // Elapsed time: 1.8557 seconds for 1000 iteration
}

fn what_is_a_string() {

    String and &str is a collection of bytes !!! Literally
    

    // What is inside of str
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "𓅱".as_bytes());

    // Ok what to do then : )
    let word_str = "안녕!";
    println!("Word is {} bytes and also {} characters.", word_str.len(), word_str.chars().count());
    
}

fn main() {

    // Copy by value (Rust word Copy Trait)
    data_lives_inside_stack();
    data_lives_inside_heap();

    // Naive approach to solve problem with ownership, just make a clone :)
    // It's ok but not sustainable

    naive_solution_just_clone();
    cost_for_naive_solution();
    use_reference_smart_decision();

    // well you could say that a win is microseconds, haha
    // techincally yes, but it's just a string happens to be small
    // if you start to work with long strings, difference will be huge
    // clone is expensive operation, use references
    
    what_is_a_string();
}



Warm-up
Problem #1
String Concatenation with Borrowing:

Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}

Problem#2

Clone and Modify:

Given a string, clone it and modify the cloned string by appending a new word. Print both the original string and the cloned, modified string to show that the original has not been changed.

rust
Copy code
fn clone_and_modify(s: &String) -> String {
    // Your code here
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}

https://leetcode.com/problems/fibonacci-number/

Time to code:

https://leetcode.com/problems/move-zeroes/


https://leetcode.com/problems/first-bad-version/