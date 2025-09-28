// Pico Proof Marketplace Library
// Decentralized marketplace for Pico zkVM proof generation services

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRequest {
    pub id: Uuid,
    pub requester_id: String,
    pub program_hash: String,
    pub input_data: Vec<u8>,
    pub backend: String,
    pub max_price: u64, // in wei or smallest token unit
    pub deadline: SystemTime,
    pub priority: Priority,
    pub status: RequestStatus,
    pub created_at: SystemTime,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequestStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResponse {
    pub id: Uuid,
    pub request_id: Uuid,
    pub prover_id: String,
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<u64>,
    pub generation_time: Duration,
    pub cycles: u64,
    pub memory_usage: u64,
    pub proof_size: usize,
    pub price: u64,
    pub submitted_at: SystemTime,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProverProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supported_backends: Vec<String>,
    pub supported_programs: Vec<String>,
    pub pricing: PricingModel,
    pub performance_stats: PerformanceStats,
    pub reputation_score: f64,
    pub total_proofs: u64,
    pub successful_proofs: u64,
    pub average_generation_time: Duration,
    pub uptime: f64,
    pub created_at: SystemTime,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModel {
    pub model_type: PricingType,
    pub base_price: u64,
    pub per_cycle_price: u64,
    pub per_mb_price: u64,
    pub priority_multiplier: HashMap<String, f64>,
    pub backend_multiplier: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingType {
    Fixed,
    PerCycle,
    PerMB,
    Hybrid,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub average_generation_time: Duration,
    pub average_verification_time: Duration,
    pub success_rate: f64,
    pub throughput: f64, // proofs per hour
    pub max_concurrent_requests: usize,
    pub current_load: f64,
    pub uptime_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    pub total_requests: u64,
    pub total_proofs_generated: u64,
    pub active_provers: usize,
    pub average_proof_time: Duration,
    pub average_price: u64,
    pub total_volume: u64,
    pub success_rate: f64,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: Uuid,
    pub request_id: Uuid,
    pub prover_id: String,
    pub price: u64,
    pub estimated_time: Duration,
    pub submitted_at: SystemTime,
    pub status: BidStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BidStatus {
    Active,
    Accepted,
    Rejected,
    Expired,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub proof_id: Uuid,
    pub valid: bool,
    pub verification_time: Duration,
    pub verifier_id: String,
    pub verified_at: SystemTime,
    pub error_message: Option<String>,
}

pub struct PicoProofMarketplace {
    requests: HashMap<Uuid, ProofRequest>,
    responses: HashMap<Uuid, ProofResponse>,
    provers: HashMap<String, ProverProfile>,
    bids: HashMap<Uuid, Vec<Bid>>,
    verifications: HashMap<Uuid, VerificationResult>,
    stats: MarketplaceStats,
}

impl PicoProofMarketplace {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
            responses: HashMap::new(),
            provers: HashMap::new(),
            bids: HashMap::new(),
            verifications: HashMap::new(),
            stats: MarketplaceStats {
                total_requests: 0,
                total_proofs_generated: 0,
                active_provers: 0,
                average_proof_time: Duration::from_millis(0),
                average_price: 0,
                total_volume: 0,
                success_rate: 0.0,
                last_updated: SystemTime::now(),
            },
        }
    }

    pub fn register_prover(&mut self, profile: ProverProfile) -> Result<(), String> {
        if self.provers.contains_key(&profile.id) {
            return Err("Prover already registered".to_string());
        }

        if profile.supported_backends.is_empty() {
            return Err("Prover must support at least one backend".to_string());
        }

        self.provers.insert(profile.id.clone(), profile);
        self.stats.active_provers = self.provers.len();
        Ok(())
    }

    pub fn submit_request(&mut self, request: ProofRequest) -> Result<Uuid, String> {
        if request.deadline <= SystemTime::now() {
            return Err("Deadline must be in the future".to_string());
        }

        if request.input_data.is_empty() {
            return Err("Input data cannot be empty".to_string());
        }

        // Find eligible provers
        let eligible_provers: Vec<_> = self.provers
            .values()
            .filter(|p| p.supported_backends.contains(&request.backend))
            .collect();

        if eligible_provers.is_empty() {
            return Err(format!("No provers available for backend: {}", request.backend));
        }

        let request_id = request.id;
        self.requests.insert(request_id, request);
        self.stats.total_requests += 1;
        self.stats.last_updated = SystemTime::now();

        Ok(request_id)
    }

    pub fn submit_bid(&mut self, bid: Bid) -> Result<Uuid, String> {
        let request = self.requests.get(&bid.request_id)
            .ok_or("Request not found")?;

        if request.status != RequestStatus::Pending {
            return Err("Request is no longer accepting bids".to_string());
        }

        if bid.estimated_time.as_secs() > 3600 { // Max 1 hour
            return Err("Estimated time too long".to_string());
        }

        let bids = self.bids.entry(bid.request_id).or_insert_with(Vec::new);
        bids.push(bid.clone());

        Ok(bid.id)
    }

    pub fn accept_bid(&mut self, request_id: Uuid, bid_id: Uuid) -> Result<(), String> {
        let request = self.requests.get_mut(&request_id)
            .ok_or("Request not found")?;

        if request.status != RequestStatus::Pending {
            return Err("Request is no longer accepting bids".to_string());
        }

        let bids = self.bids.get_mut(&request_id)
            .ok_or("No bids found for request")?;

        let bid = bids.iter_mut()
            .find(|b| b.id == bid_id)
            .ok_or("Bid not found")?;

        bid.status = BidStatus::Accepted;
        request.status = RequestStatus::InProgress;

        // Reject other bids
        for other_bid in bids.iter_mut() {
            if other_bid.id != bid_id {
                other_bid.status = BidStatus::Rejected;
            }
        }

        Ok(())
    }

    pub fn submit_proof(&mut self, response: ProofResponse) -> Result<Uuid, String> {
        let request = self.requests.get(&response.request_id)
            .ok_or("Request not found")?;

        if request.status != RequestStatus::InProgress {
            return Err("Request is not in progress".to_string());
        }

        if response.proof_data.is_empty() {
            return Err("Proof data cannot be empty".to_string());
        }

        self.responses.insert(response.id, response.clone());
        self.stats.total_proofs_generated += 1;
        self.stats.total_volume += response.price;

        // Update request status
        if let Some(req) = self.requests.get_mut(&response.request_id) {
            req.status = RequestStatus::Completed;
        }

        // Update prover stats
        if let Some(prover) = self.provers.get_mut(&response.prover_id) {
            prover.total_proofs += 1;
            prover.successful_proofs += 1;
            prover.reputation_score = (prover.successful_proofs as f64 / prover.total_proofs as f64) * 100.0;
        }

        self.update_stats();
        Ok(response.id)
    }

    pub fn verify_proof(&mut self, proof_id: Uuid, result: VerificationResult) -> Result<(), String> {
        let response = self.responses.get_mut(&proof_id)
            .ok_or("Proof not found")?;

        response.verified = result.valid;
        self.verifications.insert(proof_id, result);

        // Update prover reputation
        if let Some(prover) = self.provers.get_mut(&response.prover_id) {
            if !response.verified {
                prover.successful_proofs = prover.successful_proofs.saturating_sub(1);
            }
            prover.reputation_score = (prover.successful_proofs as f64 / prover.total_proofs as f64) * 100.0;
        }

        self.update_stats();
        Ok(())
    }

    pub fn get_best_bids(&self, request_id: Uuid, count: usize) -> Vec<&Bid> {
        let bids = self.bids.get(&request_id).map_or_else(Vec::new, |bids| {
            let mut sorted_bids = bids.iter().collect::<Vec<_>>();
            sorted_bids.sort_by(|a, b| {
                // Sort by price first, then by estimated time
                a.price.cmp(&b.price).then_with(|| a.estimated_time.cmp(&b.estimated_time))
            });
            sorted_bids.into_iter().take(count).collect()
        });
        bids
    }

    pub fn get_prover_rankings(&self) -> Vec<(&String, &ProverProfile)> {
        let mut rankings: Vec<_> = self.provers.iter().collect();
        rankings.sort_by(|a, b| {
            b.1.reputation_score.partial_cmp(&a.1.reputation_score).unwrap()
                .then_with(|| b.1.successful_proofs.cmp(&a.1.successful_proofs))
        });
        rankings
    }

    pub fn get_marketplace_stats(&self) -> &MarketplaceStats {
        &self.stats
    }

    pub fn search_provers(&self, backend: Option<&str>, max_price: Option<u64>, min_reputation: Option<f64>) -> Vec<&ProverProfile> {
        self.provers.values()
            .filter(|p| {
                if let Some(b) = backend {
                    if !p.supported_backends.contains(&b.to_string()) {
                        return false;
                    }
                }
                if let Some(price) = max_price {
                    if p.pricing.base_price > price {
                        return false;
                    }
                }
                if let Some(reputation) = min_reputation {
                    if p.reputation_score < reputation {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn get_request_status(&self, request_id: Uuid) -> Option<&RequestStatus> {
        self.requests.get(&request_id).map(|r| &r.status)
    }

    pub fn get_proof_by_request(&self, request_id: Uuid) -> Option<&ProofResponse> {
        self.responses.values()
            .find(|r| r.request_id == request_id)
    }

    pub fn cancel_request(&mut self, request_id: Uuid, requester_id: &str) -> Result<(), String> {
        let request = self.requests.get_mut(&request_id)
            .ok_or("Request not found")?;

        if request.requester_id != requester_id {
            return Err("Unauthorized".to_string());
        }

        if !matches!(request.status, RequestStatus::Pending | RequestStatus::InProgress) {
            return Err("Cannot cancel request in current status".to_string());
        }

        request.status = RequestStatus::Cancelled;
        Ok(())
    }

    pub fn get_pending_requests(&self) -> Vec<&ProofRequest> {
        self.requests.values()
            .filter(|r| matches!(r.status, RequestStatus::Pending))
            .collect()
    }

    pub fn export_marketplace_data(&self, format: &str) -> Result<String, Box<dyn std::error::Error>> {
        match format {
            "json" => {
                let data = serde_json::json!({
                    "stats": self.stats,
                    "provers": self.provers,
                    "requests": self.requests,
                    "responses": self.responses
                });
                Ok(serde_json::to_string_pretty(&data)?)
            }
            "csv" => {
                let mut csv = String::new();
                csv.push_str("prover_id,name,reputation_score,total_proofs,successful_proofs,base_price\n");
                
                for prover in self.provers.values() {
                    csv.push_str(&format!(
                        "{},{},{:.2},{},{},{}\n",
                        prover.id,
                        prover.name,
                        prover.reputation_score,
                        prover.total_proofs,
                        prover.successful_proofs,
                        prover.pricing.base_price
                    ));
                }
                
                Ok(csv)
            }
            _ => Err("Unsupported format".into()),
        }
    }

    fn update_stats(&mut self) {
        if !self.responses.is_empty() {
            let total_time: Duration = self.responses.values()
                .map(|r| r.generation_time)
                .sum();
            self.stats.average_proof_time = total_time / self.responses.len() as u32;

            let total_price: u64 = self.responses.values()
                .map(|r| r.price)
                .sum();
            self.stats.average_price = total_price / self.responses.len() as u64;

            let verified_count = self.responses.values()
                .filter(|r| r.verified)
                .count();
            self.stats.success_rate = (verified_count as f64 / self.responses.len() as f64) * 100.0;
        }

        self.stats.last_updated = SystemTime::now();
    }
}
