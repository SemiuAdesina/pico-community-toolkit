// Pico Monitoring Dashboard Library
// Web-based monitoring and analytics for Pico zkVM deployments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMetrics {
    pub deployment_id: String,
    pub program_name: String,
    pub status: DeploymentStatus,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub total_proofs_generated: u64,
    pub total_proofs_verified: u64,
    pub average_proof_time: Duration,
    pub total_cycles: u64,
    pub memory_usage: u64,
    pub backend: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Running,
    Stopped,
    Error,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetrics {
    pub proof_id: String,
    pub deployment_id: String,
    pub generation_time: Duration,
    pub verification_time: Duration,
    pub proof_size: usize,
    pub cycles: u64,
    pub memory_usage: u64,
    pub backend: String,
    pub timestamp: SystemTime,
    pub status: ProofStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofStatus {
    Generated,
    Verified,
    Failed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub deployment_id: String,
    pub time_period: Duration,
    pub total_proofs: u64,
    pub successful_proofs: u64,
    pub failed_proofs: u64,
    pub average_generation_time: Duration,
    pub average_verification_time: Duration,
    pub throughput: f64, // proofs per second
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub deployments: Vec<DeploymentHealth>,
    pub system_resources: SystemResources,
    pub alerts: Vec<Alert>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentHealth {
    pub deployment_id: String,
    pub status: HealthStatus,
    pub uptime: Duration,
    pub last_activity: SystemTime,
    pub error_rate: f64,
    pub response_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResources {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub available_memory: u64,
    pub available_disk: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub deployment_id: Option<String>,
    pub timestamp: SystemTime,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub refresh_interval: Duration,
    pub retention_period: Duration,
    pub alert_thresholds: AlertThresholds,
    pub endpoints: Vec<String>,
    pub authentication: Option<AuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub max_response_time: Duration,
    pub max_error_rate: f64,
    pub max_cpu_usage: f64,
    pub max_memory_usage: f64,
    pub min_throughput: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub enabled: bool,
    pub username: String,
    pub password_hash: String,
}

pub struct PicoMonitoringDashboard {
    deployments: HashMap<String, DeploymentMetrics>,
    proof_metrics: Vec<ProofMetrics>,
    config: DashboardConfig,
    alerts: Vec<Alert>,
}

impl PicoMonitoringDashboard {
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            deployments: HashMap::new(),
            proof_metrics: Vec::new(),
            config,
            alerts: Vec::new(),
        }
    }

    pub fn register_deployment(&mut self, metrics: DeploymentMetrics) {
        self.deployments.insert(metrics.deployment_id.clone(), metrics);
    }

    pub fn record_proof_metrics(&mut self, metrics: ProofMetrics) {
        self.proof_metrics.push(metrics);
        
        // Keep only recent metrics based on retention period
        let cutoff = SystemTime::now() - self.config.retention_period;
        self.proof_metrics.retain(|m| m.timestamp > cutoff);
    }

    pub fn get_deployment_metrics(&self, deployment_id: &str) -> Option<&DeploymentMetrics> {
        self.deployments.get(deployment_id)
    }

    pub fn get_performance_stats(&self, deployment_id: &str, time_period: Duration) -> PerformanceStats {
        let cutoff = SystemTime::now() - time_period;
        let relevant_proofs: Vec<_> = self.proof_metrics
            .iter()
            .filter(|p| p.deployment_id == deployment_id && p.timestamp > cutoff)
            .collect();

        let total_proofs = relevant_proofs.len() as u64;
        let successful_proofs = relevant_proofs.iter()
            .filter(|p| matches!(p.status, ProofStatus::Verified))
            .count() as u64;
        let failed_proofs = total_proofs - successful_proofs;

        let avg_generation_time = if !relevant_proofs.is_empty() {
            let total_time: Duration = relevant_proofs.iter()
                .map(|p| p.generation_time)
                .sum();
            total_time / relevant_proofs.len() as u32
        } else {
            Duration::from_millis(0)
        };

        let avg_verification_time = if !relevant_proofs.is_empty() {
            let total_time: Duration = relevant_proofs.iter()
                .map(|p| p.verification_time)
                .sum();
            total_time / relevant_proofs.len() as u32
        } else {
            Duration::from_millis(0)
        };

        let throughput = if time_period.as_secs() > 0 {
            total_proofs as f64 / time_period.as_secs() as f64
        } else {
            0.0
        };

        PerformanceStats {
            deployment_id: deployment_id.to_string(),
            time_period,
            total_proofs,
            successful_proofs,
            failed_proofs,
            average_generation_time: avg_generation_time,
            average_verification_time: avg_verification_time,
            throughput,
            cpu_usage: 45.0, // Placeholder
            memory_usage: 60.0, // Placeholder
            disk_usage: 25.0, // Placeholder
        }
    }

    pub fn get_system_health(&self) -> SystemHealth {
        let deployment_healths: Vec<_> = self.deployments
            .values()
            .map(|d| {
                let uptime = d.end_time
                    .unwrap_or_else(SystemTime::now)
                    .duration_since(d.start_time)
                    .unwrap_or_default();

                let error_rate = if d.total_proofs_generated > 0 {
                    (d.total_proofs_generated - d.total_proofs_verified) as f64 / d.total_proofs_generated as f64
                } else {
                    0.0
                };

                let status = if error_rate > 0.1 {
                    HealthStatus::Critical
                } else if error_rate > 0.05 {
                    HealthStatus::Warning
                } else {
                    HealthStatus::Healthy
                };

                DeploymentHealth {
                    deployment_id: d.deployment_id.clone(),
                    status,
                    uptime,
                    last_activity: SystemTime::now(),
                    error_rate,
                    response_time: d.average_proof_time,
                }
            })
            .collect();

        let overall_status = if deployment_healths.iter().any(|h| matches!(h.status, HealthStatus::Critical)) {
            HealthStatus::Critical
        } else if deployment_healths.iter().any(|h| matches!(h.status, HealthStatus::Warning)) {
            HealthStatus::Warning
        } else if deployment_healths.is_empty() {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        };

        let system_resources = SystemResources {
            cpu_usage: 45.0,
            memory_usage: 60.0,
            disk_usage: 25.0,
            network_usage: 10.0,
            available_memory: 8 * 1024 * 1024 * 1024, // 8GB
            available_disk: 100 * 1024 * 1024 * 1024, // 100GB
        };

        SystemHealth {
            overall_status,
            deployments: deployment_healths,
            system_resources,
            alerts: self.alerts.clone(),
            timestamp: SystemTime::now(),
        }
    }

    pub fn create_alert(&mut self, severity: AlertSeverity, message: String, deployment_id: Option<String>) {
        let alert = Alert {
            id: format!("alert_{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),
            severity,
            message,
            deployment_id,
            timestamp: SystemTime::now(),
            resolved: false,
        };

        self.alerts.push(alert);
    }

    pub fn check_thresholds(&mut self) {
        let deployment_ids: Vec<String> = self.deployments.keys().cloned().collect();
        
        for deployment_id in deployment_ids {
            let stats = self.get_performance_stats(&deployment_id, Duration::from_secs(300)); // 5 minutes

            // Check response time threshold
            if stats.average_generation_time > self.config.alert_thresholds.max_response_time {
                self.create_alert(
                    AlertSeverity::Warning,
                    format!("High response time for deployment {}: {:?}", deployment_id, stats.average_generation_time),
                    Some(deployment_id.clone()),
                );
            }

            // Check error rate threshold
            let error_rate = if stats.total_proofs > 0 {
                stats.failed_proofs as f64 / stats.total_proofs as f64
            } else {
                0.0
            };

            if error_rate > self.config.alert_thresholds.max_error_rate {
                self.create_alert(
                    AlertSeverity::Critical,
                    format!("High error rate for deployment {}: {:.2}%", deployment_id, error_rate * 100.0),
                    Some(deployment_id.clone()),
                );
            }

            // Check throughput threshold
            if stats.throughput < self.config.alert_thresholds.min_throughput && stats.total_proofs > 10 {
                self.create_alert(
                    AlertSeverity::Warning,
                    format!("Low throughput for deployment {}: {:.2} proofs/sec", deployment_id, stats.throughput),
                    Some(deployment_id.clone()),
                );
            }
        }
    }

    pub fn resolve_alert(&mut self, alert_id: &str) -> bool {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
            true
        } else {
            false
        }
    }

    pub fn get_all_deployments(&self) -> Vec<&DeploymentMetrics> {
        self.deployments.values().collect()
    }

    pub fn get_recent_proofs(&self, count: usize) -> Vec<ProofMetrics> {
        let mut proofs = self.proof_metrics.clone();
        proofs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        proofs.into_iter().take(count).collect()
    }

    pub fn export_metrics(&self, format: &str) -> Result<String, Box<dyn std::error::Error>> {
        match format {
            "json" => {
                let data = serde_json::json!({
                    "deployments": self.deployments,
                    "recent_proofs": self.get_recent_proofs(100),
                    "system_health": self.get_system_health(),
                    "alerts": self.alerts
                });
                Ok(serde_json::to_string_pretty(&data)?)
            }
            "prometheus" => {
                // Generate Prometheus metrics format
                let mut metrics = String::new();
                
                for deployment in self.deployments.values() {
                    metrics.push_str(&format!(
                        "pico_deployment_total_proofs{{deployment_id=\"{}\",backend=\"{}\"}} {}\n",
                        deployment.deployment_id, deployment.backend, deployment.total_proofs_generated
                    ));
                    
                    metrics.push_str(&format!(
                        "pico_deployment_verified_proofs{{deployment_id=\"{}\",backend=\"{}\"}} {}\n",
                        deployment.deployment_id, deployment.backend, deployment.total_proofs_verified
                    ));
                    
                    metrics.push_str(&format!(
                        "pico_deployment_total_cycles{{deployment_id=\"{}\",backend=\"{}\"}} {}\n",
                        deployment.deployment_id, deployment.backend, deployment.total_cycles
                    ));
                }
                
                Ok(metrics)
            }
            _ => Err("Unsupported format".into()),
        }
    }
}
