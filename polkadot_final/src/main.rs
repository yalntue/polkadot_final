use std::process::Command;
use std::io;

fn main() -> io::Result<()> {
    // Define the project names and GitHub repository URLs
    let substrate_repo = "https://github.com/substrate-developer-hub/substrate-node-template";
    let frontend_repo = "https://github.com/substrate-developer-hub/substrate-front-end-template";

    // Clone the Substrate project
    clone_repository(substrate_repo, "substrate-node-template")?;

    // Change directory to the Substrate project
    change_directory("substrate-node-template")?;

    // Create a new git branch
    create_git_branch("my-new-branch")?;

    // Build the Substrate project
    build_substrate_project()?;

    // Start the Substrate node
    start_substrate_node()?;

    // Clone the front-end project
    clone_repository(frontend_repo, "substrate-front-end-template")?;

    // Change directory to the front-end project
    change_directory("substrate-front-end-template")?;

    // Install front-end dependencies
    install_frontend_dependencies()?;

    // Start the front-end
    start_frontend()?;

    // Provide information to the user
    println!("Front end is running at http://localhost:8000/");
    println!("Interact with the Substrate node through the front end.");

    // Wait for the user to stop the node
    println!("Press Enter to stop the Substrate node...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Terminate the Substrate node
    terminate_substrate_node()?;

    Ok(())
}

fn clone_repository(repo_url: &str, _folder_name: &str) -> io::Result<()> {
    Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .status()?;
    Ok(())
}

fn change_directory(folder_name: &str) -> io::Result<()> {
    Command::new("cd")
        .arg(folder_name)
        .status()?;
    Ok(())
}

fn create_git_branch(branch_name: &str) -> io::Result<()> {
    Command::new("git")
        .arg("switch")
        .arg("-c")
        .arg(branch_name)
        .status()?;
    Ok(())
}

fn build_substrate_project() -> io::Result<()> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()?;
    Ok(())
}

fn start_substrate_node() -> io::Result<()> {
    Command::new("./target/release/node-template")
        .arg("--dev")
        .spawn()?;
    Ok(())
}

fn install_frontend_dependencies() -> io::Result<()> {
    Command::new("yarn")
        .arg("install")
        .status()?;
    Ok(())
}

fn start_frontend() -> io::Result<()> {
    Command::new("yarn")
        .arg("start")
        .spawn()?;
    Ok(())
}

fn terminate_substrate_node() -> io::Result<()> {
    Command::new("pkill")
        .arg("node-template")
        .status()?;
    Ok(())
}
