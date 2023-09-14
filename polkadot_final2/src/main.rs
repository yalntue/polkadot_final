use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};

struct Config {
    node_name: String,
    data_path: String,
    chain_spec: String,
    node_port: u16,
    ws_port: u16,
    rpc_port: u16,
    telemetry_url: String,
}

#[derive(Serialize)]
struct NodeConfig {
    config: Config,
}

fn main() -> io::Result<()> {
    let config = Config {
        node_name: "MyNode".to_string(),
        data_path: "/tmp/mynode".to_string(),
        chain_spec: "./customSpec.json".to_string(),
        node_port: 30333,
        ws_port: 9944,
        rpc_port: 9933,
        telemetry_url: "wss://telemetry.polkadot.io/submit/0".to_string(),
    };

    let node_config = NodeConfig { config };

    let json_config = serde_json::to_string_pretty(&node_config)?;

    let mut file = File::create("node_config.json")?;
    file.write_all(json_config.as_bytes())?;

    Ok(())
}

