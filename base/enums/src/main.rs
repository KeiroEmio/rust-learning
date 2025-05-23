#![allow(dead_code)]
// 网络连接状态枚举
enum NetworkConnection {
    Connected(ConnectionInfo),
    Disconnected(DisconnectReason),
    Connecting { retry_count: u32, timeout: u64 },
    Failed(NetworkError),
}

// 连接信息结构体
#[derive(Debug)]
struct ConnectionInfo {
    ip: String,
    port: u16,
    protocol: Protocol,
}

#[derive(Debug)]
enum Protocol {
    TCP,
    UDP,
    HTTP,
}

// 断开原因枚举
enum DisconnectReason {
    UserRequested,
    ConnectionLost,
    Timeout,
}

// 网络错误枚举
enum NetworkError {
    InvalidAddress,
    PortInUse,
    ConnectionRefused,
}

fn handle_connection(connection: NetworkConnection) {
    match connection {
        NetworkConnection::Connected(info) => {
            println!("已连接到 {}:{} 使用 {:?} 协议",
                    info.ip, info.port, info.protocol);
        }
        NetworkConnection::Disconnected(reason) => {
            match reason {
                DisconnectReason::UserRequested => println!("用户主动断开连接"),
                DisconnectReason::ConnectionLost => println!("连接丢失"),
                DisconnectReason::Timeout => println!("连接超时"),
            }
        }
        NetworkConnection::Connecting { retry_count, timeout } => {
            println!("正在尝试连接... 重试次数: {}, 超时时间: {}ms", 
                    retry_count, timeout);
        }
        NetworkConnection::Failed(error) => {
            match error {
                NetworkError::InvalidAddress => println!("无效的地址"),
                NetworkError::PortInUse => println!("端口已被占用"),
                NetworkError::ConnectionRefused => println!("连接被拒绝"),
            }
        }
    }
}

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 测试不同的连接状态
    let connected = NetworkConnection::Connected(ConnectionInfo {
        ip: String::from("192.168.1.1"),
        port: 8080,
        protocol: Protocol::HTTP,
    });

    let disconnected = NetworkConnection::Disconnected(
        DisconnectReason::ConnectionLost
    );

    let connecting = NetworkConnection::Connecting {
        retry_count: 3,
        timeout: 5000,
    };

    let failed = NetworkConnection::Failed(
        NetworkError::PortInUse
    );

    // 处理各种状态
    handle_connection(connected);
    handle_connection(disconnected);
    handle_connection(connecting);
    handle_connection(failed);


    // -====================================================================================
    //official example
    // -
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}