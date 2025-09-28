use clap::{Parser, Subcommand};
use pico_backend_comparison::{BackendComparison, ComparisonConfig, BackendType};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pico-compare")]
#[command(about = "Compare Pico zkVM proving backends performance")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compare backends for a specific ELF file
    Compare {
        /// Path to the ELF file
        #[arg(short, long)]
        elf: PathBuf,
        
        /// Input data for the program
        #[arg(short, long)]
        input: String,
        
        /// Backends to compare (kb=KoalaBear, bb=BabyBear, m31=Mersenne31)
        #[arg(short, long, default_values = ["kb", "bb"])]
        backends: Vec<String>,
        
        /// Number of iterations for averaging
        #[arg(short = 'i', long, default_value = "3")]
        iterations: usize,
        
        /// Output format (json, table, csv)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    
    /// Run comprehensive benchmark suite
    Benchmark {
        /// Directory containing ELF files to benchmark
        #[arg(short, long)]
        directory: PathBuf,
        
        /// Output directory for results
        #[arg(short, long, default_value = "./benchmark-results")]
        output: PathBuf,
    },
    
    /// Analyze existing benchmark results
    Analyze {
        /// Path to benchmark results JSON file
        #[arg(short, long)]
        results: PathBuf,
        
        /// Generate recommendations
        #[arg(long)]
        recommend: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Compare { elf, input, backends, iterations, format } => {
            let backend_types: Vec<BackendType> = backends
                .iter()
                .map(|b| match b.as_str() {
                    "kb" => BackendType::KoalaBear,
                    "bb" => BackendType::BabyBear,
                    "m31" => BackendType::Mersenne31,
                    _ => panic!("Invalid backend: {}. Use kb, bb, or m31", b),
                })
                .collect();
            
            let config = ComparisonConfig {
                elf_path: elf,
                input_data: input,
                backends: backend_types,
                iterations,
                output_format: format,
            };
            
            let mut comparison = BackendComparison::new(config);
            comparison.run().await?;
        }
        
        Commands::Benchmark { directory, output } => {
            println!("Running comprehensive benchmark suite...");
            println!("Directory: {:?}", directory);
            println!("Output: {:?}", output);
            // TODO: Implement benchmark suite
        }
        
        Commands::Analyze { results, recommend } => {
            println!("Analyzing benchmark results...");
            println!("Results file: {:?}", results);
            println!("Generate recommendations: {}", recommend);
            // TODO: Implement analysis
        }
    }
    
    Ok(())
}