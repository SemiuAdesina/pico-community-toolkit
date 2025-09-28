# Pico Monitoring Dashboard

Web-based monitoring and analytics dashboard for Pico zkVM deployments with real-time metrics and alerting.

## Overview

This tool provides comprehensive monitoring and analytics for Pico zkVM deployments, including real-time metrics, system health monitoring, performance tracking, and automated alerting.

## Features

- **Real-time Monitoring**: Live metrics and performance data
- **System Health Dashboard**: Overall system status and health indicators
- **Performance Analytics**: Detailed performance metrics and trends
- **Alert Management**: Automated alerting with configurable thresholds
- **Deployment Tracking**: Monitor multiple deployments simultaneously
- **Web Interface**: Browser-based dashboard for easy access
- **Export Capabilities**: Export data in multiple formats

## Installation

```bash
cd monitoring-dashboard
cargo build --release
```

## Usage

### Start Monitoring Server

```bash
# Start dashboard server on default port
cargo run -- serve

# Start with custom host and port
cargo run -- serve --host 0.0.0.0 --port 8080

# Start with configuration file
cargo run -- serve --config monitoring_config.json
```

### Add Deployments to Monitor

```bash
# Add deployment to monitoring
cargo run -- add --id deployment_1 --program fibonacci --backend koalabear

# Add with custom settings
cargo run -- add --id my_app --program my_program.elf --backend babybear --version 1.0.0
```

### Get Deployment Metrics

```bash
# Get metrics for specific deployment
cargo run -- metrics --deployment deployment_1 --period 300

# Get metrics in specific format
cargo run -- metrics --deployment deployment_1 --format json --output metrics.json
```

### Check System Health

```bash
# Check overall system health
cargo run -- health

# Get health status in JSON format
cargo run -- health --format json
```

### Manage Alerts

```bash
# List all alerts
cargo run -- alerts

# List only unresolved alerts
cargo run -- alerts --unresolved

# Resolve specific alert
cargo run -- resolve --alert-id alert_123
```

## Command Reference

### `serve`
Start the monitoring dashboard web server.

**Options:**
- `--port`: Port to listen on (default: 8080)
- `--host`: Host to bind to (default: 127.0.0.1)
- `--config`: Configuration file path

### `add`
Add a deployment to monitoring.

**Options:**
- `--id`: Unique deployment identifier
- `--program`: Program name or path
- `--backend`: Backend being used

### `metrics`
Get deployment metrics and statistics.

**Options:**
- `--deployment`: Deployment ID to get metrics for
- `--period`: Time period in seconds (default: 300)
- `--format`: Output format (text, json, prometheus)
- `--output`: Output file path

### `health`
Get system health status.

**Options:**
- `--format`: Output format (text, json)
- `--detailed`: Include detailed health information

### `alerts`
List and manage alerts.

**Options:**
- `--unresolved`: Show only unresolved alerts
- `--severity`: Filter by severity level
- `--format`: Output format (text, json)

## Web Interface

### Dashboard URL
Access the web interface at: `http://localhost:8080`

### Available Endpoints

#### Health and Status
- `GET /health` - System health status
- `GET /status` - Overall system status

#### Metrics and Data
- `GET /metrics` - All metrics in Prometheus format
- `GET /metrics/{deployment}` - Deployment-specific metrics
- `GET /deployments` - List all deployments
- `GET /deployments/{id}` - Specific deployment details

#### Alerts
- `GET /alerts` - List all alerts
- `GET /alerts/unresolved` - Unresolved alerts only
- `POST /alerts/{id}/resolve` - Resolve specific alert

## Contributing

1. Add new metric types in `src/metrics.rs`
2. Implement alerting logic in `src/alerts.rs`
3. Enhance web interface in `src/web/`
4. Add new export formats in `src/export.rs`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `pico-vm`: Pico virtual machine
- `axum`: Web framework for the dashboard
- `tokio`: Async runtime
- `chrono`: Date and time handling
- `serde`: Serialization framework
- `clap`: Command-line argument parsing
