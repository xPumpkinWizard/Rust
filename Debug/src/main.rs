fn main() {
    let data_address: *const u8;
    // Scope start
    {
        let sensitive_data = "password".to_string();
        data_address = sensitive_data.as_ptr();
        println!("Sensitive data in scope: {}", sensitive_data);
    }
    // Scope end

    // Simulate some operations to keep the program running
    println!("Out of scope");

    // Placeholder to prevent program from exiting immediately;

    std::thread::sleep(std::time::Duration::from_secs(1));
}