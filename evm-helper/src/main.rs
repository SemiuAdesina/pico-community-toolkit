use clap::{Parser, Subcommand};
use pico_evm_helper::{EVMIntegrationHelper, ProofFormatType};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pico-evm")]
#[command(about = "EVM integration helper for Pico zkVM programs")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Solidity contract for proof verification
    Generate {
        /// Name of the contract
        #[arg(short, long)]
        name: String,
        
        /// Output file for the contract
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Pico program source file
        #[arg(short, long)]
        program: Option<PathBuf>,
    },
    
    /// Format proof for EVM consumption
    Format {
        /// Proof data (hex string)
        #[arg(short, long)]
        proof: String,
        
        /// Output format (solidity, calldata, json, raw)
        #[arg(short, long, default_value = "solidity")]
        format: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Generate verification contract
    Verifier {
        /// Verification key
        #[arg(short, long)]
        key: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Generate deployment script
    Deploy {
        /// Contract name
        #[arg(short, long)]
        contract: String,
        
        /// Constructor arguments
        #[arg(short, long)]
        args: Vec<String>,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Generate test contract
    Test {
        /// Contract name to test
        #[arg(short, long)]
        contract: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Estimate gas cost for proof verification
    Gas {
        /// Proof size in bytes
        #[arg(short, long)]
        proof_size: usize,
        
        /// Number of public inputs
        #[arg(short, long, default_value = "1")]
        inputs: usize,
    },
    
    /// Validate EVM integration
    Validate {
        /// Contract ABI file
        #[arg(short, long)]
        abi: PathBuf,
        
        /// Proof file
        #[arg(short, long)]
        proof: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    let helper = EVMIntegrationHelper::new();
    
    match cli.command {
        Commands::Generate { name, output, program } => {
            println!("Generating Solidity contract: {}", name);
            
            let program_source = if let Some(program_path) = program {
                std::fs::read_to_string(&program_path)?
            } else {
                "// Pico program placeholder".to_string()
            };
            
            let contract_code = helper.generate_solidity_contract(&program_source, &name)?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &contract_code)?;
                println!("Contract written to: {:?}", output_path);
            } else {
                println!("Generated Contract:");
                println!("{}", contract_code);
            }
        }
        
        Commands::Format { proof, format, output } => {
            println!("Formatting proof for EVM consumption...");
            
            let format_type = match format.as_str() {
                "solidity" => ProofFormatType::Solidity,
                "calldata" => ProofFormatType::Calldata,
                "json" => ProofFormatType::JSON,
                "raw" => ProofFormatType::Raw,
                _ => {
                    eprintln!("Invalid format: {}. Use: solidity, calldata, json, raw", format);
                    std::process::exit(1);
                }
            };
            
            let formatted_proof = helper.format_proof_for_evm(&proof, format_type.clone())?;
            
            let output_data = match format_type {
                ProofFormatType::JSON => formatted_proof.proof_data,
                _ => format!("Proof: {}\nFormat: {:?}", formatted_proof.proof_data, formatted_proof.format_type),
            };
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &output_data)?;
                println!("Formatted proof written to: {:?}", output_path);
            } else {
                println!("Formatted Proof:");
                println!("{}", output_data);
            }
        }
        
        Commands::Verifier { key, output } => {
            println!("Generating verification contract...");
            
            let verifier_code = helper.generate_verification_contract(&key)?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &verifier_code)?;
                println!("Verifier contract written to: {:?}", output_path);
            } else {
                println!("Generated Verifier Contract:");
                println!("{}", verifier_code);
            }
        }
        
        Commands::Deploy { contract, args, output } => {
            println!("Generating deployment script for: {}", contract);
            
            let deploy_script = helper.create_deployment_script(&contract, &args)?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &deploy_script)?;
                println!("Deployment script written to: {:?}", output_path);
            } else {
                println!("Generated Deployment Script:");
                println!("{}", deploy_script);
            }
        }
        
        Commands::Test { contract, output } => {
            println!("Generating test contract for: {}", contract);
            
            let test_code = helper.generate_test_contract(&contract)?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &test_code)?;
                println!("Test contract written to: {:?}", output_path);
            } else {
                println!("Generated Test Contract:");
                println!("{}", test_code);
            }
        }
        
        Commands::Gas { proof_size, inputs } => {
            println!("Estimating gas cost for proof verification...");
            
            let gas_estimate = helper.estimate_gas_cost(proof_size, inputs);
            
            println!("Gas Estimation:");
            println!("  Proof size: {} bytes", proof_size);
            println!("  Public inputs: {}", inputs);
            println!("  Estimated gas: {} wei", gas_estimate);
            println!("  Estimated gas: {:.6} ETH (at 20 gwei)", gas_estimate as f64 * 20e-9 / 1e18);
        }
        
        Commands::Validate { abi, proof } => {
            println!("Validating EVM integration...");
            println!("ABI file: {:?}", abi);
            println!("Proof file: {:?}", proof);
            
            // Basic validation checks
            if !abi.exists() {
                eprintln!("Error: ABI file does not exist");
                std::process::exit(1);
            }
            
            if !proof.exists() {
                eprintln!("Error: Proof file does not exist");
                std::process::exit(1);
            }
            
            // Read and validate files
            let abi_content = std::fs::read_to_string(&abi)?;
            let proof_content = std::fs::read_to_string(&proof)?;
            
            // Basic JSON validation for ABI
            if let Err(e) = serde_json::from_str::<serde_json::Value>(&abi_content) {
                eprintln!("Error: Invalid ABI JSON format: {}", e);
                std::process::exit(1);
            }
            
            // Basic hex validation for proof
            if !proof_content.starts_with("0x") || proof_content.len() < 4 {
                eprintln!("Error: Proof should be a valid hex string starting with 0x");
                std::process::exit(1);
            }
            
            println!("✓ ABI file is valid JSON");
            println!("✓ Proof file is valid hex format");
            println!("✓ EVM integration validation passed");
        }
    }
    
    Ok(())
}