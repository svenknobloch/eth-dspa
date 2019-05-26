use std::fs::create_dir_all;

use bincode::deserialize;
use zmq::{Context, SocketType, SNDMORE};

use dspa_lib::records::{CommentRecord, LikeRecord, PostRecord, StreamRecord};
use dspa_lib::{Topic, DATA_SOCKET, SOURCE_SOCKET};

fn main() {
    // Create context
    let ctx = Context::new();
    create_dir_all("/tmp/dspa").expect("Failed to create temp dir");

    // Create publish socket
    let send_socket = ctx.socket(SocketType::PUB).unwrap();
    send_socket
        .bind(&format!("ipc://{}", DATA_SOCKET))
        .expect("Send socket failed to bind");

    // Create pull socket
    let recv_socket = ctx.socket(SocketType::PULL).unwrap();
    recv_socket
        .bind(&format!("ipc://{}", SOURCE_SOCKET))
        .expect("Recv socket failed to bind");

    // Listen for message pairs (topic, data)
    while let (Ok(topic), Ok(data)) = (recv_socket.recv_bytes(0), recv_socket.recv_bytes(0)) {
        // Forward message with given topic
        send_socket.send(&topic, SNDMORE).unwrap();
        let topic = String::from_utf8(topic).unwrap();

        match topic.as_str() {
            "post" => {
                let record = deserialize::<PostRecord>(&data).unwrap();
                println!(
                    "{} record {{ timestamp: {}, id: {:?} }}",
                    topic,
                    record.timestamp(),
                    record.id()
                );
            }
            "comment" => {
                let record = deserialize::<CommentRecord>(&data).unwrap();
                println!(
                    "{} record {{ timestamp: {}, id: {:?} }}",
                    topic,
                    record.timestamp(),
                    record.id()
                );
            }
            "like" => {
                let record = deserialize::<LikeRecord>(&data).unwrap();
                println!(
                    "{} record {{ timestamp: {}, id: {:?} }}",
                    topic,
                    record.timestamp(),
                    record.id()
                );
            }
            _ => {}
        }
        send_socket.send(data, 0).unwrap();

        // println!("{:?}", topic);
        if topic == Topic::EOS.to_string() {
            break;
        }
    }
}
