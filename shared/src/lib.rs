use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    pub version: String,
    pub host: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct OverviewResponse {
    pub logged_user: String,
    pub storage_used: String,
    pub counter: Counter,
    pub host: HostInfo,
    pub throughput: Throughput,
    pub mounted: Vec<MountedDeviceInfo>,
}

#[derive(Serialize)]
pub struct MountedDeviceInfo {
    pub mount_path: String,
    pub device: String,
    #[serde(rename = "type")]
    pub fs_type: String,
    pub total: String,
    pub used: String,
    pub free: String,
}

#[derive(Serialize)]
pub struct Counter {
    pub users: UserCounter,
    pub clients: ClientCounter,
    pub objects: ObjectCounter,
}

#[derive(Serialize)]
pub struct UserCounter {
    pub total: usize,
    pub added_this_week: usize,
}

#[derive(Serialize)]
pub struct ClientCounter {
    pub total: usize,
    pub active: usize,
}

#[derive(Serialize)]
pub struct ObjectCounter {
    pub total: usize,
    pub buckets: usize,
}

#[derive(Serialize)]
pub struct HostInfo {
    pub os: String,
    pub kernel: String,
    pub architecture: String,
    pub hostname: String,
    pub uptime: String,
    pub cpu: String,
    pub cpu_cores: u16,
    pub load_average: String,
    pub used_storage: u64,
    pub total_storage: u64,
}

#[derive(Serialize)]
pub struct Throughput {
    pub ingres: String,
    pub egres: String,
    pub req_per_second: u32,
    pub latency_p99: String,
}
