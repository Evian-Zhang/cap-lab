use serde::{Deserialize, Serialize};

pub const DAEMON_PORT: u16 = 9876;
pub const EXECUTE_PATH: &'static str = "execute";
pub const PERMANENTLY_REMOVE_PATH: &'static str = "permanently_remove";
pub const TEMPORARILY_REMOVE_PATH: &'static str = "temporarily_remove";
pub const TEMPORARILY_RECALIM_PATH: &'static str = "temporarily_reclaim";

#[derive(Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub command: String,
}

#[derive(Serialize, Deserialize)]
pub struct PermanentlyRemoveRequest {
    pub capability: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemporarilyRemoveRequest {
    pub capability: String,
}

#[derive(Serialize, Deserialize)]
pub struct TemporarilyReclaimRequest {
    pub capability: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub return_value: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize, Deserialize)]
pub struct CapabilityResponse {
    pub is_ok: bool,
}
