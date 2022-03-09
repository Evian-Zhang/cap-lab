use actix_web::{web, App, HttpServer, Responder};
use cap_lab_interface::{
    CapabilityResponse, ExecuteRequest, ExecuteResponse, PermanentlyRemoveRequest,
    TemporarilyReclaimRequest, TemporarilyRemoveRequest, DAEMON_PORT, EXECUTE_PATH,
    PERMANENTLY_REMOVE_PATH, TEMPORARILY_RECALIM_PATH, TEMPORARILY_REMOVE_PATH,
};
use caps::{CapSet, Capability};
use std::process::Stdio;
use tokio::process::Command;

async fn execute(execute_request: web::Json<ExecuteRequest>) -> impl Responder {
    let child_process = Command::new("sh")
        .arg("-c")
        .arg(&execute_request.command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let output = child_process.wait_with_output().await.unwrap();
    let return_value = output.status.code().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    web::Json(ExecuteResponse {
        return_value,
        stdout,
        stderr,
    })
}

async fn permanently_remove(
    permanently_remove_request: web::Json<PermanentlyRemoveRequest>,
) -> impl Responder {
    let capability = match permanently_remove_request.capability.parse::<Capability>() {
        Ok(capability) => capability,
        Err(error) => {
            return web::Json(CapabilityResponse {
                has_error: Some(format!("Failed to identify capability: {}", error)),
            });
        }
    };
    if let Err(error) = caps::drop(None, CapSet::Permitted, capability) {
        return web::Json(CapabilityResponse {
            has_error: Some(format!("{}", error)),
        });
    }
    web::Json(CapabilityResponse { has_error: None })
}

async fn temporarily_remove(
    temporarily_remove_request: web::Json<TemporarilyRemoveRequest>,
) -> impl Responder {
    let capability = match temporarily_remove_request.capability.parse::<Capability>() {
        Ok(capability) => capability,
        Err(error) => {
            return web::Json(CapabilityResponse {
                has_error: Some(format!("Failed to identify capability: {}", error)),
            });
        }
    };
    if let Err(error) = caps::drop(None, CapSet::Ambient, capability) {
        return web::Json(CapabilityResponse {
            has_error: Some(format!("{}", error)),
        });
    }
    web::Json(CapabilityResponse { has_error: None })
}

async fn temporarily_reclaim(
    temporarily_reclaim_request: web::Json<TemporarilyReclaimRequest>,
) -> impl Responder {
    let capability = match temporarily_reclaim_request.capability.parse::<Capability>() {
        Ok(capability) => capability,
        Err(error) => {
            return web::Json(CapabilityResponse {
                has_error: Some(format!("Failed to identify capability: {}", error)),
            });
        }
    };
    if let Err(error) = caps::raise(None, CapSet::Ambient, capability) {
        return web::Json(CapabilityResponse {
            has_error: Some(format!("{}", error)),
        });
    }
    web::Json(CapabilityResponse { has_error: None })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    // see https://unix.stackexchange.com/a/580466
    let permit = caps::read(None, CapSet::Permitted).unwrap();
    caps::set(None, CapSet::Inheritable, &permit).unwrap();
    caps::set(None, CapSet::Ambient, &permit).unwrap();
    HttpServer::new(|| {
        App::new()
            .route(EXECUTE_PATH, web::post().to(execute))
            .route(PERMANENTLY_REMOVE_PATH, web::post().to(permanently_remove))
            .route(TEMPORARILY_REMOVE_PATH, web::post().to(temporarily_remove))
            .route(
                TEMPORARILY_RECALIM_PATH,
                web::post().to(temporarily_reclaim),
            )
    })
    .workers(1)
    .worker_max_blocking_threads(1)
    .bind(("localhost", DAEMON_PORT))?
    .run()
    .await
}
