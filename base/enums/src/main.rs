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
}