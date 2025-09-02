use mavlink::{MavMessage, MavConnection};

/// Pretty-print incoming MAVLink message
pub fn print_message(msg: &MavMessage) {
    match msg {
        MavMessage::HEARTBEAT(heartbeat) => {
            println!(
                "HEARTBEAT: Type={}, Autopilot={}, Mode={:?}",
                heartbeat.type_, heartbeat.autopilot, heartbeat.base_mode
            );
        }
        MavMessage::ATTITUDE(attitude) => {
            println!(
                "ATTITUDE: roll={}, pitch={}, yaw={}",
                attitude.roll, attitude.pitch, attitude.yaw
            );
        }
        _ => {
            println!("Received: {:?}", msg);
        }
    }
}
