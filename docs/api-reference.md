# API Reference

## Overview

This document provides comprehensive API documentation for all components of the Pico Community Toolkit.

## Backend Comparison Tool

### CLI Interface
```bash
pico-compare [OPTIONS] --program <PROGRAM> --input <INPUT>
```

#### Options
- `--program <PROGRAM>`: Path to the RISC-V ELF program
- `--input <INPUT>`: Input data for the program
- `--backends <BACKENDS>`: Comma-separated list of backends to compare
- `--output <OUTPUT>`: Output file for comparison results
- `--iterations <ITERATIONS>`: Number of iterations per backend (default: 5)

#### Example
```bash
pico-compare --program ./fibonacci.elf --input "100" --backends "koalabear,babybear,m31" --output results.json
```

### Library API
```rust
use pico_backend_comparison::{ComparisonConfig, ComparisonReport, PicoBackendComparison};

// Create comparison configuration
let config = ComparisonConfig {
    program_path: PathBuf::from("program.elf"),
    input_data: "100".to_string(),
    backends: vec!["koalabear".to_string(), "babybear".to_string()],
    iterations: 5,
};

// Run comparison
let mut comparison = PicoBackendComparison::new();
let report = comparison.run_comparison(config)?;

// Access results
println!("Best backend: {}", report.summary.recommended_backend);
```

## CLI Extensions

### CLI Interface
```bash
pico-ext <COMMAND>
```

#### Commands

##### Analyze
```bash
pico-ext analyze --elf <ELF> [--format <FORMAT>] [--detailed]
```
- `--elf <ELF>`: Path to ELF file
- `--format <FORMAT>`: Output format (json, yaml, table)
- `--detailed`: Include detailed analysis

##### Optimize
```bash
pico-ext optimize --elf <ELF> [--backend <BACKEND>] [--output <OUTPUT>]
```
- `--elf <ELF>`: Path to ELF file
- `--backend <BACKEND>`: Target backend for optimization
- `--output <OUTPUT>`: Output file path

##### Test
```bash
pico-ext test --directory <DIRECTORY> [--inputs <INPUTS>] [--output <OUTPUT>]
```
- `--directory <DIRECTORY>`: Directory containing test cases
- `--inputs <INPUTS>`: Input data file
- `--output <OUTPUT>`: Test results output file

##### Profile
```bash
pico-ext profile --elf <ELF> --input <INPUT> [--iterations <ITERATIONS>] [--memory]
```
- `--elf <ELF>`: Path to ELF file
- `--input <INPUT>`: Input data
- `--iterations <ITERATIONS>`: Number of profiling iterations
- `--memory`: Enable memory profiling

### Library API
```rust
use pico_cli_extensions::{ElfAnalyzer, Optimizer, BatchTester, Profiler};

// ELF Analysis
let analyzer = ElfAnalyzer::new();
analyzer.analyze(&elf_path, "json", true)?;

// Optimization
let optimizer = Optimizer::new();
optimizer.optimize(&elf_path, "koalabear", Some(&output_path))?;

// Batch Testing
let tester = BatchTester::new();
tester.run_tests(&test_dir, &inputs_path, &output_path)?;

// Profiling
let profiler = Profiler::new();
profiler.profile(&elf_path, "100", 10, true)?;
```

## Debugger & Profiler

### CLI Interface
```bash
pico-debug <COMMAND>
```

#### Commands

##### Debug
```bash
pico-debug debug --program <PROGRAM> [--input <INPUT>] [--breakpoint <BREAKPOINT>]
```
- `--program <PROGRAM>`: Path to program
- `--input <INPUT>`: Input data
- `--breakpoint <BREAKPOINT>`: Set breakpoint at address

##### Interactive
```bash
pico-debug interactive --program <PROGRAM>
```
- `--program <PROGRAM>`: Path to program

##### Analyze
```bash
pico-debug analyze --program <PROGRAM> [--output <OUTPUT>]
```
- `--program <PROGRAM>`: Path to program
- `--output <OUTPUT>`: Analysis output file

##### Memory
```bash
pico-debug memory --program <PROGRAM> --address <ADDRESS>
```
- `--program <PROGRAM>`: Path to program
- `--address <ADDRESS>`: Memory address to inspect

##### Profile
```bash
pico-debug profile --program <PROGRAM> --input <INPUT> [--iterations <ITERATIONS>] [--memory]
```
- `--program <PROGRAM>`: Path to program
- `--input <INPUT>`: Input data
- `--iterations <ITERATIONS>`: Number of profiling iterations
- `--memory`: Enable memory profiling

### Library API
```rust
use pico_debugger::{PicoDebugger, Profiler, DebugSession, Breakpoint};

// Create debugger instance
let mut debugger = PicoDebugger::new();

// Start debug session
let session = debugger.start_session("program.elf")?;

// Set breakpoints
debugger.set_breakpoint(Breakpoint {
    address: 0x1000,
    condition: None,
})?;

// Step through execution
debugger.step()?;

// Inspect memory
let value = debugger.get_memory_value(0x2000)?;

// Profiling
let mut profiler = Profiler::new();
profiler.start_profiling();
// ... run program ...
let result = profiler.stop_profiling()?;
```

## EVM Integration Helper

### CLI Interface
```bash
pico-evm <COMMAND>
```

#### Commands

##### Generate
```bash
pico-evm generate --name <NAME> [--output <OUTPUT>] [--program <PROGRAM>]
```
- `--name <NAME>`: Contract name
- `--output <OUTPUT>`: Output file path
- `--program <PROGRAM>`: Pico program path

##### Format
```bash
pico-evm format --proof <PROOF> --format <FORMAT> [--output <OUTPUT>]
```
- `--proof <PROOF>`: Proof file path
- `--format <FORMAT>`: Target format (solidity, calldata, json, raw)
- `--output <OUTPUT>`: Output file path

##### Deploy
```bash
pico-evm deploy --contract <CONTRACT> [--network <NETWORK>] [--private-key <KEY>]
```
- `--contract <CONTRACT>`: Contract file path
- `--network <NETWORK>`: Target network
- `--private-key <KEY>`: Private key for deployment

##### Verify
```bash
pico-evm verify --contract <CONTRACT> --proof <PROOF> [--public-inputs <INPUTS>]
```
- `--contract <CONTRACT>`: Contract address
- `--proof <PROOF>`: Proof data
- `--public-inputs <INPUTS>`: Public inputs

### Library API
```rust
use pico_evm_helper::{EVMIntegrationHelper, EVMContract, ProofFormatType};

// Create helper instance
let helper = EVMIntegrationHelper::new();

// Generate Solidity contract
let contract_code = helper.generate_solidity_contract("MyProgram", "MyVerifier")?;

// Format proof for EVM
let formatted_proof = helper.format_proof_for_evm(&proof, ProofFormatType::SolidityCalldata)?;

// Estimate gas costs
let gas_estimate = helper.estimate_gas_cost(&contract_code)?;
```

## Testing Framework

### CLI Interface
```bash
pico-test <COMMAND>
```

#### Commands

##### Run
```bash
pico-test run --directory <DIRECTORY> [--verbose]
```
- `--directory <DIRECTORY>`: Test directory path
- `--verbose`: Verbose output

##### Generate
```bash
pico-test generate --test-name <NAME> --program-path <PATH>
```
- `--test-name <NAME>`: Test case name
- `--program-path <PATH>`: Program path

##### Benchmark
```bash
pico-test benchmark --directory <DIRECTORY> [--output <OUTPUT>]
```
- `--directory <DIRECTORY>`: Test directory
- `--output <OUTPUT>`: Benchmark results file

##### Validate
```bash
pico-test validate --directory <DIRECTORY>
```
- `--directory <DIRECTORY>`: Test directory

##### Coverage
```bash
pico-test coverage [--report-file <FILE>]
```
- `--report-file <FILE>`: Coverage report file

##### Performance
```bash
pico-test performance --program <PROGRAM> [--iterations <ITERATIONS>]
```
- `--program <PROGRAM>`: Program path
- `--iterations <ITERATIONS>`: Number of iterations

### Library API
```rust
use pico_testing_framework::{PicoTestingFramework, TestCase, TestResult};

// Create framework instance
let mut framework = PicoTestingFramework::new();

// Load test cases
let test_cases = framework.load_test_cases(&test_dir)?;

// Run tests
for test_case in test_cases {
    let result = framework.run_test(test_case);
    println!("Test {}: {:?}", result.test_name, result.status);
}

// Generate test template
let template = framework.generate_test_template("my_test", &program_path);

// Run benchmarks
let benchmark_results = framework.run_benchmarks(test_cases)?;

// Generate coverage report
let coverage_report = framework.generate_coverage_report(&test_results);
```

## Monitoring Dashboard

### CLI Interface
```bash
pico-monitor <COMMAND>
```

#### Commands

##### Start
```bash
pico-monitor start [--port <PORT>] [--host <HOST>]
```
- `--port <PORT>`: Server port (default: 8080)
- `--host <HOST>`: Server host (default: localhost)

##### Deploy
```bash
pico-monitor deploy --name <NAME> --program <PROGRAM> [--backend <BACKEND>]
```
- `--name <NAME>`: Deployment name
- `--program <PROGRAM>`: Program path
- `--backend <BACKEND>`: Backend type

##### Metrics
```bash
pico-monitor metrics --deployment <DEPLOYMENT> [--timeframe <TIMEFRAME>]
```
- `--deployment <DEPLOYMENT>`: Deployment ID
- `--timeframe <TIMEFRAME>`: Time range for metrics

##### Alerts
```bash
pico-monitor alerts [--unresolved]
```
- `--unresolved`: Show only unresolved alerts

##### Health
```bash
pico-monitor health
```

### Library API
```rust
use pico_monitoring_dashboard::{PicoMonitoringDashboard, DeploymentMetrics};

// Create dashboard instance
let mut dashboard = PicoMonitoringDashboard::new();

// Deploy program
let deployment = dashboard.deploy_program("my_app", "program.elf", "koalabear")?;

// Record metrics
dashboard.record_proof_metrics(&deployment.id, proof_metrics)?;

// Check system health
let health = dashboard.get_system_health();

// Get alerts
let alerts = dashboard.get_alerts(true); // unresolved only
```

## Proof Marketplace

### CLI Interface
```bash
pico-market <COMMAND>
```

#### Commands

##### Register
```bash
pico-market register --name <NAME> --contact <CONTACT> [--backends <BACKENDS>] [--base-price <PRICE>]
```
- `--name <NAME>`: Prover name
- `--contact <CONTACT>`: Contact information
- `--backends <BACKENDS>`: Supported backends
- `--base-price <PRICE>`: Base price per proof

##### Request
```bash
pico-market request --program-id <ID> --input-data <DATA> [--backend <BACKEND>] [--priority <PRIORITY>]
```
- `--program-id <ID>`: Program identifier
- `--input-data <DATA>`: Input data
- `--backend <BACKEND>`: Preferred backend
- `--priority <PRIORITY>`: Request priority

##### Provers
```bash
pico-market provers [--backend <BACKEND>] [--max-price <PRICE>] [--min-reputation <REP>]
```
- `--backend <BACKEND>`: Filter by backend
- `--max-price <PRICE>`: Maximum price
- `--min-reputation <REP>`: Minimum reputation

##### Stats
```bash
pico-market stats
```

### Library API
```rust
use pico_proof_marketplace::{PicoProofMarketplace, ProofRequest, ProverProfile};

// Create marketplace instance
let mut marketplace = PicoProofMarketplace::new();

// Register as prover
let prover_id = marketplace.register_prover(ProverProfile {
    name: "My Prover".to_string(),
    contact: "contact@example.com".to_string(),
    supported_backends: vec!["koalabear".to_string()],
    base_price: 1000,
    // ... other fields
})?;

// Submit proof request
let request = ProofRequest {
    program_id: "fibonacci".to_string(),
    input_data: "100".to_string(),
    preferred_backend: Some("koalabear".to_string()),
    // ... other fields
};
let request_id = marketplace.submit_request(request)?;

// Search provers
let provers = marketplace.search_provers(Some("koalabear"), Some(2000), Some(0.8));
```

## Data Types

### Common Types
```rust
// Performance metrics
pub struct PerformanceMetrics {
    pub cycles: u64,
    pub memory_usage: u64,
    pub proof_size: u64,
    pub proof_generation_time: Duration,
}

// Test result
pub struct TestResult {
    pub test_name: String,
    pub status: TestStatus,
    pub duration: Duration,
    pub error_message: Option<String>,
    pub actual_output: Option<String>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

// Deployment metrics
pub struct DeploymentMetrics {
    pub id: String,
    pub name: String,
    pub program_path: String,
    pub backend: String,
    pub status: DeploymentStatus,
    pub total_proofs: u64,
    pub successful_proofs: u64,
    pub failed_proofs: u64,
    pub average_generation_time: Duration,
    pub throughput: f64,
}

// Proof metrics
pub struct ProofMetrics {
    pub id: String,
    pub deployment_id: String,
    pub generation_time: Duration,
    pub proof_size: u64,
    pub status: ProofStatus,
    pub backend_used: String,
    pub timestamp: SystemTime,
}
```

## Error Types

### Common Errors
```rust
#[derive(Debug, thiserror::Error)]
pub enum ToolkitError {
    #[error("Program not found: {path}")]
    ProgramNotFound { path: String },
    
    #[error("Invalid input data: {data}")]
    InvalidInput { data: String },
    
    #[error("Backend error: {backend}")]
    BackendError { backend: String },
    
    #[error("Proof generation failed: {reason}")]
    ProofGenerationFailed { reason: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

## Configuration

### Global Configuration
```toml
# config.toml
[defaults]
backend = "koalabear"
iterations = 5
timeout = 300

[logging]
level = "info"
format = "json"

[monitoring]
enabled = true
port = 8080
```

### Component-Specific Configuration
Each component can have its own configuration file:
- `backend-comparison/config.toml`
- `cli-extensions/config.toml`
- `debugger/config.toml`
- etc.

---

*This API reference is automatically generated and updated with each release. For the latest documentation, always refer to the most recent version.*
