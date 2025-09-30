# Agin Sensors

A modular, high-performance sensor data management system built in Rust that collects, processes, and stores sensor data from various sources.

## Overview

Agin Sensors is designed as a flexible, scalable solution for IoT sensor data collection and management. The system uses a modular architecture that allows easy addition of new data sources, databases, and data processing modifiers through a trait-based plugin system.

## Features

- **Multi-Protocol Support**: Built-in connectors for MQTT, Socket.IO, and Modbus
- **Database Flexibility**: Support for multiple time-series databases (InfluxDB, _TimescaleDB in progress_)
- **Organization-Based Filtering**: Route sensor data to different organizations based on MAC addresses, tokens, or other metadata
- **Data Processing Pipeline**: Apply custom modifiers to transform and process sensor data in real-time
- **Configuration-Driven**: YAML-based configuration for connectors, databases, and organizations
- **Buffering & Aggregation**: Built-in buffering system for data aggregation and batch processing
- **Docker Support**: Complete containerized deployment with Docker Compose

## Architecture

The system is built with a modular architecture consisting of several core components:

### Core Components

- **`aginsensors_core`**: Core traits and abstractions for connectors, databases, and modifiers
- **`daemon`**: Main application daemon that orchestrates data flow between components
- **`modules`**: Plugin system that dynamically loads available connectors, databases, and modifiers

### Connectors

- **`connector_mqtt`**: MQTT broker connector with support for various sensor formats (BeanAir)
- **`connector_socketio`**: Socket.IO server for real-time web-based sensor data
- **`connector_modbus`**: Modbus protocol connector for industrial sensors

### Databases

- **`database_influx`**: InfluxDB time-series database integration

### Modifiers

- **`modifier_template`**: Example data processing modifier that can transform measurements

## Quick Start

### Prerequisites

- Rust 1.70+ (for building from source)
- Docker and Docker Compose (for containerized deployment)

### Running with Docker Compose

1. Clone the repository:

```bash
git clone https://github.com/aginrocks/agin-sensors.git
cd agin-sensors
```

2. Configure your sensors and organizations:

   - Edit `config/global.yaml` to configure connectors and databases
   - Edit `config/organizations.yaml` to set up data routing rules

3. Start the services:

```bash
docker-compose up -d
```

The system will start with:

- Agin Sensors daemon on ports 3000 and 3001
- InfluxDB on port 8086
- TimescaleDB on port 5432

### Building from Source

1. Install Rust: https://rustup.rs/

2. Build the project:

```bash
cargo build --release
```

3. Run the daemon:

```bash
cargo run --bin daemon
```

## Configuration

### Global Configuration (`config/global.yaml`)

Configure databases and connectors:

```yaml
databases:
  influx:
    type: influxdb
    url: http://influxdb:8086
    token: your-influx-token
    organization: your-org

connectors:
  mqtt:
    type: mqtt
    host: localhost
    port: 1883
    format: beanair
  socketio:
    type: socketio
    port: 3000
```

### Organizations Configuration (`config/organizations.yaml`)

Define how sensor data is routed to different organizations:

```yaml
org1:
  name: Organization One
  buffer: true
  modifiers:
    - ModifierTemplate
  filters:
    - type: macs
      macs:
        - 66:77:88:99:AA:BB
    - type: tokens
      tokens:
        - token1
  databases:
    - key: influx
      type: influxdb
      bucket: org1_bucket
```

### Filtering Rules

Data can be routed to organizations based on:

- **MAC addresses**: Route data from specific sensor hardware
- **Authentication tokens**: Route data based on API tokens
- **Topic patterns**: Route MQTT data based on topic structure
- **IP addresses**: Route data from specific network locations

## Data Flow

1. **Data Ingestion**: Connectors receive sensor data from various protocols
2. **Event Processing**: Raw data is converted to standardized `ConnectorEvent` objects
3. **Filtering**: Events are filtered and routed to appropriate organizations
4. **Buffering**: Data can be buffered for aggregation and batch processing
5. **Modification**: Custom modifiers can transform the data (calculations, unit conversions, etc.)
6. **Storage**: Processed data is written to configured databases

## Extending the System

### Adding a New Connector

1. Create a new crate in the workspace
2. Implement the `ConnectorRunner` trait
3. Use the `define_connector!` macro for configuration
4. Add your connector to `modules/src/connectors.rs`

```rust
use aginsensors_core::{connector::ConnectorRunner, define_connector};

define_connector!(
    "my_protocol",
    MyProtocol,
    config = {
        pub host: String,
        pub port: u16,
    },
    state = {}
);

impl ConnectorRunner for MyProtocol {
    fn run(&self) -> tokio::sync::mpsc::Receiver<ConnectorEvent> {
        // Implementation
    }
}
```

### Adding a New Database

1. Create a new database crate
2. Implement the `Database` trait
3. Use the `define_database!` macro
4. Add your database to `modules/src/databases.rs`

### Adding a New Modifier

1. Create a new modifier crate
2. Implement the `Modifier` trait
3. Use the `define_modifier!` macro
4. Add your modifier to `modules/src/modifiers.rs`

## API Endpoints

### Socket.IO Connector

The Socket.IO connector provides real-time communication endpoints:

- **Connection**: `ws://localhost:3000/socket.io/`
- **Events**: Send sensor data through Socket.IO events

### REST API

Basic health check endpoint:

- **GET** `/`: Returns application version and status

## Monitoring and Logging

The system uses structured logging with configurable levels:

```bash
# Set log level
export RUST_LOG=info

# Enable debug logging for specific modules
export RUST_LOG=agin_sensors=debug,connector_mqtt=trace
```

## Development

### Project Structure

```
agin-sensors/
├── aginsensors_core/     # Core traits and types
├── daemon/              # Main application
├── modules/             # Plugin loader
├── connector_*/         # Protocol connectors
├── database_*/          # Database integrations
├── modifier_*/          # Data processing modules
├── config/              # Configuration files
└── landing_page/        # Web interface (Next.js)
```

### Running Tests

```bash
cargo test
```

### Code Generation

The system uses macros to generate boilerplate code for plugins. JSON schemas are automatically generated for configuration validation.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request
