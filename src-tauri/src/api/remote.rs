use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// The part of an access request that can be consumed by the frontend.
#[derive(Clone, Serialize, Deserialize)]
pub struct AccessInfo {
    uuid: Uuid,
    name: String,
    agent: Agent,
    code: String
}

/// The part of an active connection that can be consumed by the frontend.
#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveInfo {
    uuid: Uuid,
    name: String,
    agent: Agent
}

/// Types of user agents (guessed based on User-Agent header - not a security thing).
#[derive(Clone, Serialize, Deserialize)]
pub enum Agent {
    Android,
    IPhone,
    Desktop,
    Unknown
}