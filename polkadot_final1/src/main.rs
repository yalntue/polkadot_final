use std::process::Command;
use std::thread;

fn main() {
    // Start node1's node
    let node1_handle = start_node("node1", 30333, 9945, 9933, "0000000000000000000000000000000000000000000000000000000000000001");

    // Wait for a moment to allow node1's node to start
    thread::sleep(std::time::Duration::from_secs(5));

    // Start nod2's node
    let node2_handle = start_node("node2", 30334, 9946, 9934, "");

    // Wait for user input to exit
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    // Terminate both nodes
    terminate_node(node1_handle);
    terminate_node(node2_handle);
}

fn start_node(name: &str, port: u16, ws_port: u16, rpc_port: u16, node_key: &str) -> std::process::Child {
    let base_path = format!("/tmp/{}", name);

    // Purge old chain data
    execute_command("git clone https://github.com/substrate-developer-hub/substrate-node-template", None);
    execute_command("cd substrate-node-template", None);
    execute_command("./target/release/node-template purge-chain --base-path /tmp/alice --chain local -y", None);

    // Start the local blockchain node
    let mut command = Command::new("./target/release/node-template");
    command
        .arg("--base-path")
        .arg(&base_path)
        .arg("--chain")
        .arg("local")
        .arg(format!("--{}", name))
        .arg("--port")
        .arg(port.to_string())
        .arg("--ws-port")
        .arg(ws_port.to_string())
        .arg("--rpc-port")
        .arg(rpc_port.to_string())
        .arg("--telemetry-url")
        .arg("wss://telemetry.polkadot.io/submit/0")
        .arg("--validator");

    if !node_key.is_empty() {
        command
            .arg("--node-key")
            .arg(node_key);
    }

    command
        .spawn()
        .expect("Failed to execute command")
}

fn terminate_node(mut handle: std::process::Child) {
    // Terminate the node
    handle
        .kill()
        .expect("Failed to terminate node process");
}

fn execute_command(command: &str, args: Option<&str>) {
    let mut cmd = if let Some(args) = args {
        Command::new(command)
            .arg(args)
    } else {
        Command::new(command)
    };
    
    let mut cmd_ref = &mut cmd; // Mutable reference to cmd
    
    let _ = cmd_ref.spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait for command");
}


fn fun_name1(command: &str) -> Command {
    fun_name(command)
}

fn fun_name(command: &str) -> Command {
    Command::new(command)
}
