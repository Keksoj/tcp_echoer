use bincode;
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::string::String;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CustomFrame {
    pub id: String,
    pub data: String,
}

impl CustomFrame {
    pub fn from_str(str: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            data: str.to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize::<Self>(bytes).unwrap()
    }

    pub fn print(&self) {
        println!("{}", self.data);
    }
}

impl fmt::Display for CustomFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

// sleeps from 0 to 256 * 4 milliseconds
pub async fn random_sleep() {
    let random_duration = rand::random::<u8>();
    // println!("Sleeping for {} milliseconds", random_duration);
    let duration = Duration::from_millis((random_duration as u64) * 4);
    sleep(duration).await;
}

pub fn create_socket() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142)
}