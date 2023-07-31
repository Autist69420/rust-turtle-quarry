use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Position(i32, i32, i32);

/// From client to server
#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum CCPacket {
    /// Sent when a client connects.
    ClientConnect { name: String, id: u64 },
    /// Sent when a turtle moves down
    LevelDecrease {
        old_level: i32,
        new_level: i32,
        position: Position,
    },
    /// Sent when a turtle moves up
    LevelIncrease {
        old_level: i32,
        new_level: i32,
        position: Position,
    },
}

/// From server to client
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerPacket {
    Acknowledge {
        message: String,
        ok: bool,
    },
    /// Sent every second to update the "database"
    UpdatePosition {
        new_position: Position,
        old_position: Position,
    },
}
