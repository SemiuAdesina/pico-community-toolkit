use pico_sdk::{
    client::{KoalaBearProverClient, BabyBearProverClient},
    m31_client::M31RiscvProverClient,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendType {
    KoalaBear,
    BabyBear,
    Mersenne31,
}

impl BackendType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BackendType::KoalaBear => "KoalaBear",
            BackendType::BabyBear => "BabyBear",
            BackendType::Mersenne31 => "Mersenne31",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonConfig {
    pub elf_path: PathBuf,
    pub input_data: String,
    pub backends: Vec<BackendType>,
    pub iterations: usize,
    pub output_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub backend: BackendType,
    pub proof_time: Duration,
    pub verification_time: Duration,
    pub proof_size: usize,
    pub cycles: u64,
    pub memory_usage: usize,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    pub config: ComparisonConfig,
    pub results: Vec<BenchmarkResult>,
    pub summary: ComparisonSummary,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSummary {
    pub fastest_backend: BackendType,
    pub smallest_proof: BackendType,
    pub most_efficient: BackendType,
    pub recommendations: Vec<String>,
}

pub struct BackendComparison {
    config: ComparisonConfig,
}

impl BackendComparison {
    pub fn new(config: ComparisonConfig) -> Self {
        Self { config }
    }
    
    pub async fn run(&mut self) -> Result<()> {
        println!("🚀 Starting Pico Backend Comparison");
        println!("ELF: {:?}", self.config.elf_path);
        println!("Input: {}", self.config.input_data);
        println!("Backends: {:?}", self.config.backends);
        println!("Iterations: {}", self.config.iterations);
        println!();
        
        let mut results = Vec::new();
        
        for backend in &self.config.backends {
            println!("Testing {} backend...", backend.as_str());
            
            let mut backend_results = Vec::new();
            
            for i in 1..=self.config.iterations {
                print!("  Iteration {}/{}: ", i, self.config.iterations);
                
                match self.benchmark_backend(backend).await {
                    Ok(result) => {
                        println!("✅ {:.2}s", result.proof_time.as_secs_f64());
                        backend_results.push(result);
                    }
                    Err(e) => {
                        println!("❌ Error: {}", e);
                        backend_results.push(BenchmarkResult {
                            backend: backend.clone(),
                            proof_time: Duration::ZERO,
                            verification_time: Duration::ZERO,
                            proof_size: 0,
                            cycles: 0,
                            memory_usage: 0,
                            success: false,
                            error: Some(e.to_string()),
                        });
                    }
                }
            }
            
            results.extend(backend_results);
            println!();
        }
        
        let report = self.generate_report(results);
        self.output_results(report).await?;
        
        Ok(())
    }
    
    async fn benchmark_backend(&self, backend: &BackendType) -> Result<BenchmarkResult> {
        let start_time = Instant::now();
        
        // Load ELF
        let elf = std::fs::read(&self.config.elf_path)?;
        
        // Create appropriate client based on backend
        let result = match backend {
            BackendType::KoalaBear => {
                let client = KoalaBearProverClient::new(&elf);
                let mut stdin_builder = client.new_stdin_builder();
                stdin_builder.write_slice(self.config.input_data.as_bytes());
                
                let start = Instant::now();
                let proof = client.prove_fast(stdin_builder)?;
                let proof_time = start.elapsed();
                
                BenchmarkResult {
                    backend: backend.clone(),
                    proof_time,
                    verification_time: Duration::ZERO, // TODO: Implement verification timing
                    proof_size: proof.proofs.len(),
                    cycles: 0, // TODO: Extract cycles from proof
                    memory_usage: 0, // TODO: Implement memory measurement
                    success: true,
                    error: None,
                }
            }
            BackendType::BabyBear => {
                let client = BabyBearProverClient::new(&elf);
                let mut stdin_builder = client.new_stdin_builder();
                stdin_builder.write_slice(self.config.input_data.as_bytes());
                
                let start = Instant::now();
                let proof = client.prove_fast(stdin_builder)?;
                let proof_time = start.elapsed();
                
                BenchmarkResult {
                    backend: backend.clone(),
                    proof_time,
                    verification_time: Duration::ZERO,
                    proof_size: proof.proofs.len(),
                    cycles: 0,
                    memory_usage: 0,
                    success: true,
                    error: None,
                }
            }
            BackendType::Mersenne31 => {
                let client = M31RiscvProverClient::new(&elf);
                let mut stdin_builder = client.new_stdin_builder();
                stdin_builder.write_slice(self.config.input_data.as_bytes());
                
                let start = Instant::now();
                let proof = client.prove_fast(stdin_builder)?;
                let proof_time = start.elapsed();
                
                BenchmarkResult {
                    backend: backend.clone(),
                    proof_time,
                    verification_time: Duration::ZERO,
                    proof_size: proof.proofs.len(),
                    cycles: 0,
                    memory_usage: 0,
                    success: true,
                    error: None,
                }
            }
        };
        
        Ok(result)
    }
    
    fn generate_report(&self, results: Vec<BenchmarkResult>) -> ComparisonReport {
        let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();
        
        let fastest_backend = successful_results
            .iter()
            .min_by(|a, b| a.proof_time.cmp(&b.proof_time))
            .map(|r| r.backend.clone())
            .unwrap_or_else(|| BackendType::KoalaBear);
        
        let smallest_proof = successful_results
            .iter()
            .min_by(|a, b| a.proof_size.cmp(&b.proof_size))
            .map(|r| r.backend.clone())
            .unwrap_or_else(|| BackendType::KoalaBear);
        
        let most_efficient = successful_results
            .iter()
            .min_by(|a, b| (a.proof_time.as_millis() as f64 / a.proof_size as f64)
                .partial_cmp(&(b.proof_time.as_millis() as f64 / b.proof_size as f64))
                .unwrap_or(std::cmp::Ordering::Equal))
            .map(|r| r.backend.clone())
            .unwrap_or_else(|| BackendType::KoalaBear);
        
        let mut recommendations = Vec::new();
        recommendations.push(format!("Fastest proof generation: {}", fastest_backend.as_str()));
        recommendations.push(format!("Smallest proof size: {}", smallest_proof.as_str()));
        recommendations.push(format!("Most efficient (time/size): {}", most_efficient.as_str()));
        
        ComparisonReport {
            config: self.config.clone(),
            results,
            summary: ComparisonSummary {
                fastest_backend,
                smallest_proof,
                most_efficient,
                recommendations,
            },
            timestamp: chrono::Utc::now(),
        }
    }
    
    async fn output_results(&self, report: ComparisonReport) -> Result<()> {
        match self.config.output_format.as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&report)?;
                println!("{}", json);
            }
            "table" => {
                self.print_table(&report);
            }
            "csv" => {
                self.print_csv(&report);
            }
            _ => {
                println!("Unknown output format: {}", self.config.output_format);
                self.print_table(&report);
            }
        }
        
        Ok(())
    }
    
    fn print_table(&self, report: &ComparisonReport) {
        println!("\n📊 Backend Comparison Results");
        println!("=====================================");
        
        for result in &report.results {
            if result.success {
                println!("{}: {:.2}s, {} bytes", 
                    result.backend.as_str(), 
                    result.proof_time.as_secs_f64(),
                    result.proof_size
                );
            } else {
                println!("{}: ❌ {}", result.backend.as_str(), result.error.as_ref().unwrap());
            }
        }
        
        println!("\n🎯 Recommendations:");
        for rec in &report.summary.recommendations {
            println!("  • {}", rec);
        }
    }
    
    fn print_csv(&self, report: &ComparisonReport) {
        println!("backend,proof_time_sec,proof_size_bytes,success,error");
        for result in &report.results {
            println!("{},{:.6},{},{},{}",
                result.backend.as_str(),
                result.proof_time.as_secs_f64(),
                result.proof_size,
                result.success,
                result.error.as_ref().unwrap_or(&"".to_string())
            );
        }
    }
}
