use cap_lab_interface::{
    CapabilityResponse, ExecuteRequest, ExecuteResponse, PermanentlyRemoveRequest,
    TemporarilyReclaimRequest, TemporarilyRemoveRequest, DAEMON_PORT, EXECUTE_PATH,
    PERMANENTLY_REMOVE_PATH, TEMPORARILY_RECALIM_PATH, TEMPORARILY_REMOVE_PATH,
};
use const_format::formatcp;
use hyper::{
    body::{self, Buf},
    Body, Client, Method, Request,
};
use structopt::StructOpt;

const BASE_URI: &'static str = formatcp!("http://localhost:{}", DAEMON_PORT);
const EXECUTE_URI: &'static str = formatcp!("{}/{}", BASE_URI, EXECUTE_PATH);
const PERMANENTLY_REMOVE_URI: &'static str = formatcp!("{}/{}", BASE_URI, PERMANENTLY_REMOVE_PATH);
const TEMPORARILY_REMOVE_URI: &'static str = formatcp!("{}/{}", BASE_URI, TEMPORARILY_REMOVE_PATH);
const TEMPORARILY_RECALIM_URI: &'static str =
    formatcp!("{}/{}", BASE_URI, TEMPORARILY_RECALIM_PATH);

/// Client to control capabilities
#[derive(StructOpt)]
enum CapLabClient {
    /// Execute a certain command
    Execute {
        /// Command string
        command: String,
    },
    /// Permanently remove a capability
    PermanentlyRemove {
        /// Capability name
        capability: String,
    },
    /// Temporarily remove a capability
    TemporarilyRemove {
        /// Capability name
        capability: String,
    },
    /// Temporarily reclaim a capability
    TemporarilyReclaim {
        /// Capability name
        capability: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cmd = CapLabClient::from_args();
    let client = Client::new();
    match cmd {
        CapLabClient::Execute { command } => {
            let request = Request::builder()
                .method(Method::POST)
                .uri(EXECUTE_URI)
                .body(Body::from(
                    serde_json::to_string(&ExecuteRequest { command }).unwrap(),
                ))?;
            let response_body = body::aggregate(client.request(request).await?.into_body()).await?;
            let execute_response: ExecuteResponse =
                serde_json::from_reader(response_body.reader()).unwrap();
            println!(
                "Command executed with return value {}",
                execute_response.return_value
            );
            println!("Command stdout:");
            println!("{}", execute_response.stdout);
            println!("Command stderr:");
            println!("{}", execute_response.stderr);
        }
        CapLabClient::PermanentlyRemove { capability } => {
            let request = Request::builder()
                .method(Method::POST)
                .uri(PERMANENTLY_REMOVE_URI)
                .body(Body::from(
                    serde_json::to_string(&PermanentlyRemoveRequest { capability }).unwrap(),
                ))?;
            let response_body = body::aggregate(client.request(request).await?.into_body()).await?;
            let capability_response: CapabilityResponse =
                serde_json::from_reader(response_body.reader()).unwrap();
            if capability_response.is_ok {
                println!("Successfully modify capability.");
            } else {
                println!("Failed to modify capability.");
            }
        }
        CapLabClient::TemporarilyRemove { capability } => {
            let request = Request::builder()
                .method(Method::POST)
                .uri(TEMPORARILY_REMOVE_URI)
                .body(Body::from(
                    serde_json::to_string(&TemporarilyRemoveRequest { capability }).unwrap(),
                ))?;
            let response_body = body::aggregate(client.request(request).await?.into_body()).await?;
            let capability_response: CapabilityResponse =
                serde_json::from_reader(response_body.reader()).unwrap();
            if capability_response.is_ok {
                println!("Successfully modify capability.");
            } else {
                println!("Failed to modify capability.");
            }
        }
        CapLabClient::TemporarilyReclaim { capability } => {
            let request = Request::builder()
                .method(Method::POST)
                .uri(TEMPORARILY_RECALIM_URI)
                .body(Body::from(
                    serde_json::to_string(&TemporarilyReclaimRequest { capability }).unwrap(),
                ))?;
            let response_body = body::aggregate(client.request(request).await?.into_body()).await?;
            let capability_response: CapabilityResponse =
                serde_json::from_reader(response_body.reader()).unwrap();
            if capability_response.is_ok {
                println!("Successfully modify capability.");
            } else {
                println!("Failed to modify capability.");
            }
        }
    }
    Ok(())
}
