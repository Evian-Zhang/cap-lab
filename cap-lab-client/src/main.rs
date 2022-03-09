use structopt::StructOpt;
use hyper::{Client, Uri, http::uri::{Scheme}};

const DAEMON_PORT: u16 = 9876;
const BASE_URI: &'static str = format!("http://localhost:{}", DAEMON_PORT);
const EXECUTE_URI: &'static str = format!("{}/execute", BASE_URI);
const PERMANENTLY_REMOVE_URI: &'static str = format!("{}/permanently_remove", BASE_URI);
const TEMPORARILY_REMOVE_URI: &'static str = format!("{}/temporarily_remove", BASE_URI);
const TEMPORARILY_RECALIM_URI: &'static str = format!("{}/temporarily_reclaim", BASE_URI);


/// Client to control capabilities
#[derive(StructOpt)]
enum CapLabClient {
    /// Execute a certain command
    Execute {
        /// Command string
        command: String
    },
    /// Permanently remove a capability
    PermanentlyRemove {
        /// Capability name
        capability: String
    },
    /// Temporarily remove a capability
    TemporarilyRemove {
        /// Capability name
        capability: String
    },
    /// Temporarily reclaim a capability
    TemporarilyReclaim {
        /// Capability name
        capability: String
    }
}

#[tokio::main]
async fn main() {
    let cmd = CapLabClient::from_args();
    let client = Client::new();
    let uri = Uri::builder().scheme(Scheme::HTTP).authority(format!("localhost:{}", DAEMON_PORT)).build().unwrap();
}
