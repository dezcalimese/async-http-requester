# Async HTTP Requester

A high-performance command-line tool that asynchronously fetches cryptocurrency price data from the CoinGecko API.

## Overview

This project demonstrates efficient handling of concurrent HTTP requests in Rust, utilizing modern asynchronous programming patterns. It allows users to fetch real-time cryptocurrency price data for multiple tokens simultaneously.

## Features

- Asynchronous HTTP requests using tokio and reqwest
- Integration with CoinGecko's public API
- Concurrent price fetching for multiple cryptocurrencies
- Robust error handling and request rate limiting
- Structured logging for debugging and monitoring
- Comprehensive test coverage including integration tests

## Architecture

The application is built with the following components:

- **HTTP Client Layer**: Handles API communication using reqwest
- **Rate Limiter**: Ensures compliance with CoinGecko's API limits
- **Data Models**: Type-safe representations of API responses
- **Error Handling**: Custom error types for better error management
- **CLI Interface**: User-friendly command-line interface

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd async-http-requester
```

2. Build the project:
```bash
cargo build --release

``` 

3. The binary will be available in `target/release`

### Usage

Run the application with:
```bash
cargo run -- [cryptocurrency-symbols...]
```

Example:
```bash
cargo run -- bitcoin ethereum solana
```

## Testing

Run the test suite with:
```bash
cargo test
```
