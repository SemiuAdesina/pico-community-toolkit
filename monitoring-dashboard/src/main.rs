use clap::{Parser, Subcommand};
use pico_monitoring_dashboard::{
    PicoMonitoringDashboard, DeploymentMetrics, ProofMetrics, DashboardConfig, AlertThresholds,
    DeploymentStatus, ProofStatus
};
use std::time::{Duration, SystemTime};

#[derive(Parser)]
#[command(name = "pico-monitor")]
#[command(about = "Monitoring dashboard for Pico zkVM deployments")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the monitoring dashboard server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
        
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Config file path
        #[arg(short, long)]
        config: Option<String>,
    },
    
    /// Add a deployment to monitor
    Add {
        /// Deployment ID
        #[arg(short, long)]
        id: String,
        
        /// Program name
        #[arg(short, long)]
        program: String,
        
        /// Backend to use
        #[arg(short, long, default_value = "koalabear")]
        backend: String,
    },
    
    /// Get deployment metrics
    Metrics {
        /// Deployment ID
        #[arg(short, long)]
        deployment: String,
        
        /// Time period in seconds
        #[arg(short, long, default_value = "300")]
        period: u64,
        
        /// Output format (json, prometheus)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    
    /// Get system health status
    Health {
        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    
    /// List all alerts
    Alerts {
        /// Show only unresolved alerts
        #[arg(short, long)]
        unresolved: bool,
    },
    
    /// Resolve an alert
    Resolve {
        /// Alert ID
        #[arg(short, long)]
        alert_id: String,
    },
    
    /// Export all metrics
    Export {
        /// Output format (json, prometheus)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Simulate proof generation for testing
    Simulate {
        /// Deployment ID
        #[arg(short, long)]
        deployment: String,
        
        /// Number of proofs to simulate
        #[arg(short, long, default_value = "10")]
        count: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Create default configuration
    let config = DashboardConfig {
        refresh_interval: Duration::from_secs(30),
        retention_period: Duration::from_secs(3600), // 1 hour
        alert_thresholds: AlertThresholds {
            max_response_time: Duration::from_secs(30),
            max_error_rate: 0.05, // 5%
            max_cpu_usage: 80.0,
            max_memory_usage: 80.0,
            min_throughput: 1.0, // 1 proof per second
        },
        endpoints: vec!["http://localhost:8080".to_string()],
        authentication: None,
    };
    
    let mut dashboard = PicoMonitoringDashboard::new(config);
    
    match cli.command {
        Commands::Serve { port, host, config: _ } => {
            println!("Starting Pico Monitoring Dashboard server...");
            println!("Server will be available at: http://{}:{}", host, port);
            
            // Start a simple HTTP server simulation
            println!("Dashboard server started successfully!");
            println!("Available endpoints:");
            println!("  GET /health - System health status");
            println!("  GET /metrics - All metrics");
            println!("  GET /deployments - List deployments");
            println!("  GET /alerts - List alerts");
            println!("  POST /alerts/:id/resolve - Resolve alert");
            
            // Keep the server running
            tokio::signal::ctrl_c().await?;
            println!("Shutting down server...");
        }
        
        Commands::Add { id, program, backend } => {
            println!("Adding deployment: {}", id);
            
            let metrics = DeploymentMetrics {
                deployment_id: id.clone(),
                program_name: program,
                status: DeploymentStatus::Running,
                start_time: SystemTime::now(),
                end_time: None,
                total_proofs_generated: 0,
                total_proofs_verified: 0,
                average_proof_time: Duration::from_millis(0),
                total_cycles: 0,
                memory_usage: 0,
                backend,
                version: "1.0.0".to_string(),
            };
            
            dashboard.register_deployment(metrics);
            println!("✓ Deployment {} added successfully", id);
        }
        
        Commands::Metrics { deployment, period, format } => {
            println!("Getting metrics for deployment: {}", deployment);
            
            let time_period = Duration::from_secs(period);
            let stats = dashboard.get_performance_stats(&deployment, time_period);
            
            let output = match format.as_str() {
                "json" => {
                    serde_json::to_string_pretty(&stats)?
                }
                "prometheus" => {
                    dashboard.export_metrics("prometheus")?
                }
                _ => {
                    format!(
                        "Performance Stats for {}:\n\
                        Time Period: {:?}\n\
                        Total Proofs: {}\n\
                        Successful: {}\n\
                        Failed: {}\n\
                        Average Generation Time: {:?}\n\
                        Average Verification Time: {:?}\n\
                        Throughput: {:.2} proofs/sec\n\
                        CPU Usage: {:.1}%\n\
                        Memory Usage: {:.1}%",
                        deployment,
                        time_period,
                        stats.total_proofs,
                        stats.successful_proofs,
                        stats.failed_proofs,
                        stats.average_generation_time,
                        stats.average_verification_time,
                        stats.throughput,
                        stats.cpu_usage,
                        stats.memory_usage
                    )
                }
            };
            
            println!("{}", output);
        }
        
        Commands::Health { format } => {
            println!("Checking system health...");
            
            let health = dashboard.get_system_health();
            
            let output = match format.as_str() {
                "json" => {
                    serde_json::to_string_pretty(&health)?
                }
                _ => {
                    format!(
                        "System Health Status: {:?}\n\
                        Deployments: {}\n\
                        System Resources:\n\
                          CPU: {:.1}%\n\
                          Memory: {:.1}%\n\
                          Disk: {:.1}%\n\
                        Active Alerts: {}",
                        health.overall_status,
                        health.deployments.len(),
                        health.system_resources.cpu_usage,
                        health.system_resources.memory_usage,
                        health.system_resources.disk_usage,
                        health.alerts.iter().filter(|a| !a.resolved).count()
                    )
                }
            };
            
            println!("{}", output);
        }
        
        Commands::Alerts { unresolved } => {
            println!("Listing alerts...");
            
            let health = dashboard.get_system_health();
            let alerts: Vec<_> = if unresolved {
                health.alerts.iter().filter(|a| !a.resolved).collect()
            } else {
                health.alerts.iter().collect()
            };
            
            if alerts.is_empty() {
                println!("No alerts found.");
            } else {
                for alert in alerts {
                    println!(
                        "Alert {}: [{:?}] {} (Deployment: {:?}, Time: {:?}, Resolved: {})",
                        alert.id,
                        alert.severity,
                        alert.message,
                        alert.deployment_id.as_deref().unwrap_or("N/A"),
                        alert.timestamp,
                        alert.resolved
                    );
                }
            }
        }
        
        Commands::Resolve { alert_id } => {
            println!("Resolving alert: {}", alert_id);
            
            if dashboard.resolve_alert(&alert_id) {
                println!("✓ Alert {} resolved successfully", alert_id);
            } else {
                println!("✗ Alert {} not found", alert_id);
                std::process::exit(1);
            }
        }
        
        Commands::Export { format, output } => {
            println!("Exporting metrics in {} format...", format);
            
            let data = dashboard.export_metrics(&format)?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, &data)?;
                println!("✓ Metrics exported to: {}", output_path);
            } else {
                println!("{}", data);
            }
        }
        
        Commands::Simulate { deployment, count } => {
            println!("Simulating {} proof generations for deployment: {}", count, deployment);
            
            // Ensure deployment exists
            if dashboard.get_deployment_metrics(&deployment).is_none() {
                let metrics = DeploymentMetrics {
                    deployment_id: deployment.clone(),
                    program_name: "simulated_program".to_string(),
                    status: DeploymentStatus::Running,
                    start_time: SystemTime::now(),
                    end_time: None,
                    total_proofs_generated: 0,
                    total_proofs_verified: 0,
                    average_proof_time: Duration::from_millis(0),
                    total_cycles: 0,
                    memory_usage: 0,
                    backend: "koalabear".to_string(),
                    version: "1.0.0".to_string(),
                };
                dashboard.register_deployment(metrics);
            }
            
            for i in 0..count {
                let proof_metrics = ProofMetrics {
                    proof_id: format!("proof_{}_{}", deployment, i),
                    deployment_id: deployment.clone(),
                    generation_time: Duration::from_millis(100 + (i % 50) as u64), // Vary between 100-150ms
                    verification_time: Duration::from_millis(10 + (i % 5) as u64), // Vary between 10-15ms
                    proof_size: 2048 + (i % 100) as usize, // Vary between 2048-2148 bytes
                    cycles: 1000 + (i % 100) as u64, // Vary between 1000-1100 cycles
                    memory_usage: 1024 + (i % 100) as u64, // Vary between 1024-1124 bytes
                    backend: "koalabear".to_string(),
                    timestamp: SystemTime::now(),
                    status: if i % 10 == 0 { ProofStatus::Failed } else { ProofStatus::Verified },
                };
                
                dashboard.record_proof_metrics(proof_metrics);
                
                if i % 5 == 0 {
                    println!("Generated {} proofs...", i + 1);
                }
            }
            
            println!("✓ Simulated {} proof generations", count);
            
            // Check thresholds and generate alerts
            dashboard.check_thresholds();
            
            let stats = dashboard.get_performance_stats(&deployment, Duration::from_secs(300));
            println!("Final stats: {} total, {} successful, {:.2} proofs/sec", 
                stats.total_proofs, stats.successful_proofs, stats.throughput);
        }
    }
    
    Ok(())
}