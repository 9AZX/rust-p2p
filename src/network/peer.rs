use chrono::{DateTime, Utc};

pub struct Peer {
    status: PeerStatus,
    last_alive: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>
}

enum PeerStatus {
    IDLE,
    OUTCONNECTING,
    OUTHANDSHAKING,
    OUTALIVE,
    INHANDSHAKING,
    INALIVE,
    BANNED,
}