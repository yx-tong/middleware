use rand::Rng;
use std::{
    sync::{LazyLock, Mutex},
    time::SystemTime,
};

pub static ID_STATE16: LazyLock<Mutex<GenerateState16>> = LazyLock::new(|| Mutex::new(GenerateState16::default()));

pub struct GenerateState16 {
    machine_id: u8,
    sequence: u8,
}

impl Default for GenerateState16 {
    fn default() -> Self {
        let machine_id = match std::env::var("CLEVER_WORKER_ID") {
            Ok(o) => {
                let id = u64::from_str_radix(o.as_str(), 10).expect("`MACHINE_ID` must be a integer");
                (id % 0x100) as u8
            }
            Err(_) => rand::thread_rng().random::<u8>(),
        };
        Self { machine_id, sequence: 0 }
    }
}

impl GenerateState16 {
    /// Construct the ID: 48 bits timestamp | 8 bits machine ID | 8 bits sequence in 1ms
    pub fn generate_id(&mut self) -> u64 {
        let timestamp =
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_millis() as u64;
        // Update sequence if the timestamp is the same as the last generated
        self.sequence = self.sequence.wrapping_add(1);
        // Construct the ID
        (timestamp << 16) | ((self.machine_id as u64) << 8) | (self.sequence as u64)
    }
}
