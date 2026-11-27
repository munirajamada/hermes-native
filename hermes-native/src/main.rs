use std::time::Duration;

use hermes_gateway::{CliGrantProvider, GatewayClient};

fn probe_sessions() -> Option<usize> {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("runtime: {error}");
            return None;
        }
    };
    runtime.block_on(async {
        let client = match GatewayClient::connect(CliGrantProvider::from_env()).await {
            Ok(client) => client,
            Err(error) => {
                eprintln!("gateway connect: {error}");
                return None;
            }
        };
        if let Err(error) = client.wait_ready(Duration::from_secs(10)).await {
            eprintln!("gateway ready: {error}");
            return None;
        }
        let result = match client.request("session.list", serde_json::json!({})).await {
            Ok(result) => result,
            Err(error) => {
                eprintln!("session.list: {error}");
                return None;
            }
        };
        result
            .get("sessions")
            .and_then(serde_json::Value::as_array)
            .map(Vec::len)
    })
}

fn main() {
    let sessions = probe_sessions();
    match sessions {
        Some(count) => println!("session.list: {count} live session(s)"),
        None => eprintln!("gateway probe failed; opening the window without a session count"),
    }
    hermes_ui::run(sessions);
}
