use clap::{Parser, Subcommand};
use pico_proof_marketplace::{
    PicoProofMarketplace, ProofRequest, ProofResponse, ProverProfile, Bid, 
    Priority, RequestStatus, BidStatus, PricingModel, PricingType, PerformanceStats,
    VerificationResult
};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "pico-market")]
#[command(about = "Decentralized marketplace for Pico zkVM proof generation services")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Register as a proof provider
    Register {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        backends: String,
        #[arg(short, long, default_value = "1000000000000000")]
        base_price: u64,
    },
    /// Submit a proof generation request
    Request {
        #[arg(short, long)]
        requester: String,
        #[arg(short, long)]
        program: String,
        #[arg(short, long, default_value = "koalabear")]
        backend: String,
        #[arg(short, long, default_value = "10000000000000000")]
        max_price: u64,
        #[arg(short, long, default_value = "60")]
        deadline_minutes: u64,
        #[arg(short, long, default_value = "normal")]
        priority: String,
    },
    /// List available provers
    Provers {
        #[arg(short, long)]
        backend: Option<String>,
        #[arg(short, long)]
        max_price: Option<u64>,
        #[arg(short, long)]
        min_reputation: Option<f64>,
    },
    /// Show marketplace statistics
    Stats {
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    let mut marketplace = PicoProofMarketplace::new();
    
    // Add sample prover for demo
    let sample_prover = ProverProfile {
        id: "prover_1".to_string(),
        name: "Fast Pico Prover".to_string(),
        description: "High-performance proof generation service".to_string(),
        supported_backends: vec!["koalabear".to_string(), "babybear".to_string()],
        supported_programs: vec!["fibonacci".to_string(), "merkle".to_string()],
        pricing: PricingModel {
            model_type: PricingType::Hybrid,
            base_price: 1000000000000000,
            per_cycle_price: 1000,
            per_mb_price: 1000000000,
            priority_multiplier: HashMap::new(),
            backend_multiplier: HashMap::new(),
        },
        performance_stats: PerformanceStats {
            average_generation_time: Duration::from_millis(800),
            average_verification_time: Duration::from_millis(50),
            success_rate: 99.5,
            throughput: 10.0,
            max_concurrent_requests: 5,
            current_load: 0.3,
            uptime_percentage: 99.9,
        },
        reputation_score: 98.5,
        total_proofs: 1250,
        successful_proofs: 1244,
        average_generation_time: Duration::from_millis(800),
        uptime: 0.999,
        created_at: SystemTime::now() - Duration::from_secs(86400 * 30),
        verified: true,
    };
    
    let _ = marketplace.register_prover(sample_prover);
    
    match cli.command {
        Commands::Register { id, name, backends, base_price } => {
            println!("Registering prover: {}", id);
            let backends_list: Vec<String> = backends.split(',').map(|s| s.trim().to_string()).collect();
            
            let prover = ProverProfile {
                id: id.clone(),
                name,
                description: "Community prover".to_string(),
                supported_backends: backends_list.clone(),
                supported_programs: vec!["*".to_string()],
                pricing: PricingModel {
                    model_type: PricingType::Fixed,
                    base_price,
                    per_cycle_price: 0,
                    per_mb_price: 0,
                    priority_multiplier: HashMap::new(),
                    backend_multiplier: HashMap::new(),
                },
                performance_stats: PerformanceStats {
                    average_generation_time: Duration::from_millis(1000),
                    average_verification_time: Duration::from_millis(100),
                    success_rate: 100.0,
                    throughput: 5.0,
                    max_concurrent_requests: 3,
                    current_load: 0.0,
                    uptime_percentage: 100.0,
                },
                reputation_score: 100.0,
                total_proofs: 0,
                successful_proofs: 0,
                average_generation_time: Duration::from_millis(1000),
                uptime: 1.0,
                created_at: SystemTime::now(),
                verified: false,
            };
            
            match marketplace.register_prover(prover) {
                Ok(_) => println!("✓ Prover {} registered successfully", id),
                Err(e) => {
                    println!("✗ Failed to register prover: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Request { requester, program, backend, max_price, deadline_minutes, priority } => {
            println!("Submitting proof request...");
            
            let priority_enum = match priority.as_str() {
                "low" => Priority::Low,
                "normal" => Priority::Normal,
                "high" => Priority::High,
                "urgent" => Priority::Urgent,
                _ => {
                    println!("Invalid priority: {}. Use: low, normal, high, urgent", priority);
                    std::process::exit(1);
                }
            };
            
            let request = ProofRequest {
                id: Uuid::new_v4(),
                requester_id: requester,
                program_hash: program,
                input_data: vec![1, 2, 3, 4, 5],
                backend,
                max_price,
                deadline: SystemTime::now() + Duration::from_secs(deadline_minutes * 60),
                priority: priority_enum,
                status: RequestStatus::Pending,
                created_at: SystemTime::now(),
                metadata: HashMap::new(),
            };
            
            match marketplace.submit_request(request) {
                Ok(request_id) => {
                    println!("✓ Request submitted successfully");
                    println!("Request ID: {}", request_id);
                }
                Err(e) => {
                    println!("✗ Failed to submit request: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Provers { backend, max_price, min_reputation } => {
            println!("Available provers:");
            let provers = marketplace.search_provers(backend.as_deref(), max_price, min_reputation);
            
            if provers.is_empty() {
                println!("No provers found matching criteria.");
            } else {
                for prover in provers {
                    println!("  {} - {} (Reputation: {:.1}%, Base Price: {} wei)", 
                        prover.id, prover.name, prover.reputation_score, prover.pricing.base_price);
                    println!("    Backends: {}", prover.supported_backends.join(", "));
                }
            }
        }
        
        Commands::Stats { format } => {
            let stats = marketplace.get_marketplace_stats();
            
            let output = match format.as_str() {
                "json" => serde_json::to_string_pretty(stats)?,
                _ => format!(
                    "Marketplace Statistics:\n\
                    Total Requests: {}\n\
                    Total Proofs Generated: {}\n\
                    Active Provers: {}\n\
                    Average Proof Time: {:?}\n\
                    Average Price: {} wei\n\
                    Success Rate: {:.1}%",
                    stats.total_requests,
                    stats.total_proofs_generated,
                    stats.active_provers,
                    stats.average_proof_time,
                    stats.average_price,
                    stats.success_rate
                )
            };
            
            println!("{}", output);
        }
    }
    
    Ok(())
}