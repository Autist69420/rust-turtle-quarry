use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Position(i32, i32, i32);

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({}, {}, {})", self.0, self.1, self.2)
    }
}

/// From client to server
#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum CCPacket {
    /// Sent when a client connects.
    ClientConnect {
        name: String,
        id: u64,
        gps: Position,
    },
    /// Sent when a turtle moves down
    ///
    /// # Paramaters
    /// - `old_level` - The old layer level
    /// - `new_level` - The new layer level
    /// - `position` - Relative coords from the original GPS location
    LevelDecrease {
        old_level: i32,
        new_level: i32,
        position: Position,
    },
    /// Sent when a turtle moves up
    ///
    /// # Paramaters
    /// - `old_level` - The old layer level
    /// - `new_level` - The new layer level
    /// - `position` - Relative coords from the original GPS location
    LevelIncrease {
        old_level: i32,
        new_level: i32,
        position: Position,
    },
    /// Sent when a turtle moves
    ///
    /// # Paramaters
    /// - `old_position` - Relative coords from the original GPS location
    /// - `new_position` - Relative coords from the original GPS location
    UpdatePosition {
        new_position: Position,
        old_position: Position,
    },
}

/// From server to client
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerPacket {
    Acknowledge { message: String, ok: bool },
}
