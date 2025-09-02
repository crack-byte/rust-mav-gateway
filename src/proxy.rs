use crate::config::{AppConfig};
use mavlink::peek_reader::PeekReader;
use tokio::net::UdpSocket;
use tokio::task;
use mavlink::{connect, write_v2_msg,read_v2_msg, common::MavMessage, MavHeader};

use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MavProxy {
    cfg: AppConfig,
}

impl MavProxy {
    pub fn new(cfg: AppConfig) -> Self {
        Self { cfg }
    }

    pub async fn run(&self) {
        let source = self.cfg.mavlink.source.clone();
        let targets = self.cfg.mavlink.targets.clone();
        let bind_port = self.cfg.proxy.listen_port.unwrap_or(14560); // Add this to your config

        let mavconn = connect::<MavMessage>(&source).expect("MAVLink source failed");
        let mavconn = Arc::new(Mutex::new(mavconn)); // Share between tasks

        // Socket for forwarding TO targets
        let tx_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());

        // Socket for receiving FROM targets (reverse direction)
        let rx_socket = UdpSocket::bind(format!("0.0.0.0:{}", bind_port))
            .await
            .expect("Failed to bind reverse UDP port");

        // 🔁 TASK 1: Forward source → targets
        let mavconn_rx = Arc::clone(&mavconn);
        let targets_rx = targets.clone();
        let tx_socket = Arc::clone(&tx_socket);

        task::spawn(async move {
            loop {
                let mavconn = mavconn_rx.lock().await;
                if let Ok((_id, msg)) = mavconn.recv() {
                    let header = MavHeader {
                        system_id: 1,
                        component_id: 1,
                        sequence: 0,
                    };
                    let mut buf = Vec::new();
                    write_v2_msg(&mut buf, header, &msg).unwrap();

                    for t in &targets_rx {
                        let target_addr: SocketAddr = format!("{}:{}", t.ip, t.port).parse().unwrap();
                        let _ = tx_socket.send_to(&buf, target_addr).await;
                    }
                }
            }
        });

        // 🔁 TASK 2: Forward targets → source
        let mavconn_tx = Arc::clone(&mavconn);
        task::spawn(async move {
            let mut buf = [0u8; 512];
            loop {
                if let Ok((size, _addr)) = rx_socket.recv_from(&mut buf).await {
                    let mut cursor = Cursor::new(&buf[..size]);
                    let mut peek_reader = PeekReader::new(&mut cursor);
                    if let Ok((header, msg)) = read_v2_msg::<MavMessage, _>(&mut peek_reader) {
                        println!("Received message: {:?}", msg);
                        let mavconn = mavconn_tx.lock().await;
                        if let Err(e) = mavconn.send_default(&msg) {
                            eprintln!("Failed to send message to MAVLink source: {:?}", e);
                        }
                    }
                }
            }
        })
        .await
        .unwrap(); // Wait forever
    }
}
