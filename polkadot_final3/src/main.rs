use std::error::Error;
use substrate_subxt::{
    SignedExtrinsic,
    ClientBuilder, runtime::Contracts,
};
use substrate_subxt::UncheckedExtrinsic;
use substrate_subxt::PairSigner;
use substrate_subxt::MultiSignature;
use substrate_subxt::SignedBlock;
use substrate_subxt::Keystore;
use substrate_subxt::Client;
use substrate_subxt::DefaultNodeRuntime;

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize a Substrate node client
    let client = ClientBuilder::<DefaultNodeRuntime>::new()
        .set_url("ws://localhost:9944") // Replace with your node's WebSocket URL
        .build()
        .expect("Failed to create Substrate client");

    // Create a key pair and signer for Bob (you can use any name)
    let keystore = Keystore::from_bytes(
        // Replace with Bob's key pair bytes
        hex::decode("YOUR_NODE2_KEYPAIR_BYTES_HERE").expect("Failed to decode key pair bytes"),
        None,
    )?;

    let signer = PairSigner::<DefaultNodeRuntime, MultiSignature>::new(
        keystore.decode()?,
        None,
    );

    // Deploy the contract
    let contract_code: &[u8] = &mod
modname {
        include_bytes!("new_contract.wasm")
    };
    let contract_code = contract_code; // Replace with your contract file
    let contract_address = deploy_contract(&client, &signer, contract_code)?;

    // Interact with the contract
    let get_result = call_contract(&client, &signer, &contract_address, "get")?;
    println!("Result of get(): {:?}", get_result);

    let flip_result = call_contract(&client, &signer, &contract_address, "flip")?;
    println!("Result of flip(): {:?}", flip_result);

    Ok(())
}



fn deploy_contract(
    client: &Client<DefaultNodeRuntime>,
    signer: &PairSigner<DefaultNodeRuntime, MultiSignature>,
    contract_code: &[u8],
) -> Result<Contracts, Box<dyn Error>> {
    // Sign and send a contract deployment extrinsic
    let deploy_extrinsic = signer.call(
        Contracts::<DefaultNodeRuntime>::put_code(
            hex::encode(contract_code),
        ),
    )?;

    // Submit the extrinsic
    client.submit_and_watch_extrinsic(
        UncheckedExtrinsic {
            extrinsic: deploy_extrinsic.into(),
            signer: None,
        },
        |result| result.is_ok(),
    )?;

    // Get the deployed contract address
    let contract_info = client.query(
        Contracts::<DefaultNodeRuntime>::instantiate_with_code(
            signer.address(),
            hex::encode(contract_code),
            0,
            1000000,
            100000,
            "default",
        ),
    )?;
    
    Ok(contract_info.output)
}

fn call_contract(
    client: &Client<DefaultNodeRuntime>,
    signer: &PairSigner<DefaultNodeRuntime, MultiSignature>,
    contract_address: &str,
    method_name: &str,
) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    // Sign and send a contract call extrinsic
    let call_extrinsic = signer.call(
        Contracts::<DefaultNodeRuntime>::call(
            signer.address(),
            contract_address,
            0,
            1000000,
            100000,
            method_name.to_string(),
            Vec::new(),
        ),
    )?;

    // Submit the extrinsic
    client.submit_and_watch_extrinsic(
        UncheckedExtrinsic {
            extrinsic: call_extrinsic.into(),
            signer: None,
        },
        |result| result.is_ok(),
    )?;

    // Get the result of the contract call
    let result = client.query(
        Contracts::<DefaultNodeRuntime>::call(
            signer.address(),
            contract_address,
            0,
            1000000,
            100000,
            method_name.to_string(),
            Vec::new(),
        ),
    )?;

    Ok(result.output)
}

