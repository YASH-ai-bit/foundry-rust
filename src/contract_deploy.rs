use ethers::contract::Contract;
use ethers::prelude::{
    BlockNumber, ContractFactory, Http, LocalWallet, Middleware, Provider, Signer,
    SignerMiddleware, U256,
};
use ethers::utils::Anvil;
use ethers_solc::{
    Artifact, ConfigurableArtifacts, Project, ProjectCompileOutput, ProjectPathsConfig,
};
use eyre::Result;
use eyre::{eyre, ContextCompat};
use hex::ToHex;
use std::path::PathBuf;
use std::time::Duration;

pub type SignerDeployedContract<T> = Contract<SignerMiddleware<Provider<T>, LocalWallet>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let anvil = Anvil::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}", anvil.endpoint());

    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "Wallet first address: {}",
        first_address.encode_hex::<String>()
    );

    let provider: Provider<Http> =
        Provider::try_from(anvil.endpoint())?.interval(Duration::from_millis(10));
    let chain_id = provider.get_chainid().await?.as_u64();
    println!("Anvil started with chain_id {chain_id}");

    let project = compile("examples/").await?;

    print_project(&project).await?;

    let balance = provider.get_balance(wallet.address(), None).await?;

    println!(
        "Wallet first address {} balance: {}",
        wallet.address().encode_hex::<String>(),
        balance
    );

    let contract_name = "SimpleToken";

    let contract = project
        .into_artifacts()
        .find(|(id, _)| id.name == contract_name)
        .map(|(_, artifact)| artifact)
        .context("Contract not found")?;

    let (abi, bytecode, _) = contract.into_parts();
    let abi = abi.context("Missing abi from contract")?;
    let bytecode = bytecode.context("Missing bytecode from contract")?;

    let wallet = wallet.with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet).into();

    let factory = ContractFactory::new(abi.clone(), bytecode, client);
    let mut deployer = factory.deploy(())?;

    let gas_price = provider.get_gas_price().await?;
    deployer.tx.set_gas_price::<U256>(gas_price);

    let contract = deployer.clone().legacy().send().await?;

    println!(
        "SimpleToken contract address {}",
        contract.address().encode_hex::<String>()
    );

    Ok(())
}

pub async fn compile(root: &str) -> Result<ProjectCompileOutput<ConfigurableArtifacts>> {
    let root = PathBuf::from(root);
    if !root.exists() {
        return Err(eyre!("Project root {root:?} does not exists!"));
    }
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()?;

    let project = Project::builder()
        .paths(paths)
        .set_auto_detect(true) 
        .no_artifacts()
        .build()?;

    let output = project.compile()?;

    if output.has_compiler_errors() {
        Err(eyre!(
            "Compiling solidity project failed: {:?}",
            output.output().errors
        ))
    } else {
        Ok(output.clone())
    }
}

pub async fn print_project(project: &ProjectCompileOutput<ConfigurableArtifacts>) -> Result<()> {
    for (id, artifact) in project.artifacts() {
        let abi = artifact.abi.as_ref().context("No ABI found for artifact")?;

        println!("{}", "=".repeat(80));
        println!("CONTRACT: {:?}", id);

        let contract = &abi.abi;
        let functions = contract.functions();
        let functions = functions.cloned();
        let constructor = contract.constructor();

        if let Some(constructor) = constructor {
            let args = &constructor.inputs;
            println!("CONSTRUCTOR args: {args:?}");
        }

        for func in functions {
            let name = &func.name;
            let params = &func.inputs;
            println!("FUNCTION  {name} {params:?}");
        }
    }
    Ok(())
}
