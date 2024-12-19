// ENUM and Pattern Matching

fn using_as_a_classic_enum() {
    #[derive(PartialEq)] // this allows to define so called comparator concept
    enum FileState {
        Open,
        Closed,
    }
    
    let state = FileState::Open;

    if state == FileState::Open {
        println!("File is Open");
    } else if state == FileState::Closed {
        println!("File is Closed");
    }
}

fn enum_as_a_class() {

    
    #[derive(Debug)]
    enum ConnectionState {
        Disconnected,
        Connecting,
        Connected,
    }

    impl ConnectionState {
        fn connect(&mut self) {
            *self = ConnectionState::Connecting;
        }
    
        fn complete(&mut self) {
            *self = ConnectionState::Connected;
        }
    
        fn disconnect(&mut self) {
            *self = ConnectionState::Disconnected;
        }
    }


    let mut connection = ConnectionState::Disconnected;
    println!("Current connection state: {:?}", connection); 
    connection.connect();
    println!("Current connection state: {:?}", connection); 
    connection.complete();
    println!("Current connection state: {:?}", connection); 
    connection.disconnect();
    println!("Current connection state: {:?}", connection); 

    }

fn using_enum_as_a_field_in_struct() {

    #[derive(PartialEq)] 
    enum FileState {
        Open,
        Closed,
    }

    struct MyFile {
        data: Vec<u8>,
        state: FileState,
    }
    
    impl MyFile {
        fn new() -> MyFile {
            MyFile { data: Vec::new(), state: FileState::Open }
        }
    
        fn write(&mut self, data: &[u8]) -> bool {
            if self.state == FileState::Closed {
                return false;
            }
    
            self.data.extend(data);
            true
        }
    
        fn close(&mut self) -> bool {
            if self.state == FileState::Closed {
                return false;
            }
    
            self.state = FileState::Closed;
            true
        }
    }

    let mut file = MyFile::new();

    // Write some data to the file
    if file.write(b"Hello, world!") {
        println!("Data written to file");
    } else {
        println!("Failed to write data to file");
    }

    // Close the file
    if file.close() {
        println!("File closed");
    } else {
        println!("Failed to close file");
    }

    // Try to write more data to the file (should fail)
    if file.write(b"More data") {
        println!("Data written to file");
    } else {
        println!("Failed to write data to file");
    }

    println!("{:?}",file.data); // again bytes are at the heart of programming
    let bytes = &file.data;
    let string = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{}", string); // Output: Hello, World!

}

fn intro_to_pattern_matching() {
    // Problem: Based on name of the coin, provide how many cents it has

    #[allow(dead_code)]
    #[allow(unused)]

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin:: Dime => 10,
            Coin:: Quarter => 25,
        }
    }

    fn int_to_coin(idx: i32) -> Coin {
        match idx {
            1 => Coin::Penny,
            2 => Coin::Nickel,
            3 => Coin::Dime,
            4 => Coin::Quarter,
            _ => Coin::Penny, // _ useful to matching all other cases
    
        }
    }

    for i in 1..5 {
            println!("{}",(value_in_cents(int_to_coin(i))));
        }
    }

fn real_power_of_enum() {
    enum Transport {
        Tcp(TcpConnection),
        Udp(UdpConnection),
    }

    struct TcpConnection {
        address: String,
        port: u16,
    }

    struct UdpConnection {
        address: String,
        port: u16,
        timeout: u32,
    }


    let tcp_connection = TcpConnection {
        address: String::from("127.0.0.1"),
        port: 8080,
    };

    
    
    let transport = Transport::Tcp(tcp_connection);

    let udp_connection = UdpConnection {
        address: String::from("127.0.0.1"),
        port: 8080,
        timeout: 10,
    };

    let transport = Transport::Udp(udp_connection);

    

    match transport {
        Transport::Tcp(tcp) => {
            println!("TCP connection to {}:{}", tcp.address, tcp.port);
            }
        Transport::Udp(udp) => {
            println!("UDP connection to {}:{}:with timeout:{} sec", udp.address, udp.port, udp.timeout);
            }
        }
    }

fn main() {
    
    using_as_a_classic_enum();

    enum_as_a_class();

    using_enum_as_a_field_in_struct();

    intro_to_pattern_matching();

    real_power_of_enum();
    
}