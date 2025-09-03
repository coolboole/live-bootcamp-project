# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Learning Mode Configuration

**IMPORTANT**: This is a learning project for Rust development. When providing assistance:

- **Ask guiding questions** instead of providing complete solutions
- **Suggest investigation approaches** rather than showing all the code
- **Point to relevant documentation** and encourage reading it first
- **Explain concepts step-by-step** but let me implement the details
- **Hint at patterns** without writing the full implementation
- **Encourage experimentation** and learning from compile errors
- **Only provide complete code** when explicitly asked with "show me the solution"

For example:
- Instead of: "Here's the complete function implementation..."
- Prefer: "You'll need a function that takes X and returns Y. Consider using pattern matching on the Result type. What do you think the error cases should be?"

The goal is to develop understanding through guided discovery, not just working code.

## Project Overview

This is a Rust microservices project implementing a dual-service authentication system:
- **app-service** (port 8000): Main application server that serves protected content and validates JWT tokens
- **auth-service** (port 3000): Authentication service handling user signup, login, 2FA, and token verification

Both services are built with Axum web framework and follow clean architecture principles with domain-driven design.

## Common Development Commands

### Building
```bash
# Build both services
cd app-service && cargo build
cd ../auth-service && cargo build

# Build for release
cd app-service && cargo build --release
cd ../auth-service && cargo build --release
```

### Testing
```bash
# Run tests for both services
cd app-service && cargo test --verbose
cd ../auth-service && cargo test --verbose

# Run property-based tests (auth-service uses quickcheck)
cd auth-service && cargo test
```

### Development with Hot Reload
Install cargo-watch first: `cargo install cargo-watch`

```bash
# Run app-service with hot reload (watches src/, assets/, templates/)
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run

# Run auth-service with hot reload (watches src/, assets/)
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

### Docker Development
```bash
# Build and run both services with Docker Compose
docker compose build
docker compose up

# Use override for local development builds
docker compose -f compose.yml -f compose.override.yml up --build
```

### Linting and Formatting
```bash
# Format code
cd app-service && cargo fmt
cd auth-service && cargo fmt

# Run clippy
cd app-service && cargo clippy
cd auth-service && cargo clippy
```

## Service Architecture

### app-service
- **Purpose**: Main application serving protected content
- **Port**: 8000
- **Key Dependencies**: axum, askama (templating), reqwest (HTTP client)
- **Routes**:
  - `/` - Root page with login/logout links
  - `/protected` - Protected route requiring JWT authentication
  - `/assets/*` - Static file serving

### auth-service  
- **Purpose**: Authentication and authorization service
- **Port**: 3000
- **Key Dependencies**: axum, uuid, validator, async-trait
- **Architecture**: Clean architecture with domain/services/routes separation
- **Routes**:
  - `/signup` - User registration
  - `/login` - User authentication
  - `/verify-2fa` - Two-factor authentication verification
  - `/logout` - User logout
  - `/verify-token` - JWT token verification (internal API)

## Domain Architecture (auth-service)

The auth-service follows clean architecture principles:

```
src/
├── domain/           # Core business logic and types
│   ├── user.rs      # User entity
│   ├── email.rs     # Email value object with validation
│   ├── password.rs  # Password value object
│   ├── error.rs     # Domain errors
│   └── data_stores.rs # Storage trait definitions
├── services/        # Infrastructure implementations
│   └── hashmap_user_store.rs # In-memory user storage
├── routes/          # HTTP route handlers
│   ├── signup.rs
│   ├── login.rs
│   ├── verify_2fa.rs
│   ├── logout.rs
│   └── verify_token.rs
├── utils/           # Shared utilities and constants
│   ├── constants.rs # Application constants (JWT_COOKIE_NAME, etc.)
│   ├── auth.rs     # JWT token generation, validation, and cookie management
│   └── mod.rs      # Module exports
└── app_state/       # Application state and dependency injection
```

## Inter-Service Communication

- **app-service** validates JWT tokens by making HTTP requests to **auth-service** `/verify-token` endpoint
- Environment variables control service discovery:
  - `AUTH_SERVICE_IP`: External IP for frontend redirects (default: localhost)
  - `AUTH_SERVICE_HOST_NAME`: Internal hostname for service-to-service calls (default: 0.0.0.0)

## Testing Strategy

### Unit Tests
- Domain objects have comprehensive unit tests (email validation, password parsing)
- Property-based testing using `quickcheck` crate for validation logic
- Tests are located in `#[cfg(test)]` modules within source files

### Integration Tests
- Auth-service has integration tests for user store implementations
- Uses `fake` crate for generating test data

### Running Specific Tests
```bash
# Run tests for a specific module
cd auth-service
cargo test domain::email::tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Configuration and Environment

### Development Environment Variables
```bash
# app-service
AUTH_SERVICE_IP=localhost          # For frontend redirects
AUTH_SERVICE_HOST_NAME=localhost   # For backend API calls

# Production (Docker)
AUTH_SERVICE_IP=${DROPLET_IP}      # Set via CI/CD
```

### Production Deployment
- CI/CD via GitHub Actions (`.github/workflows/prod.yml`)
- Docker images pushed to Docker Hub (ergonomic7912/app-service, ergonomic7912/auth-service)
- Deployed to DigitalOcean droplet via SSH
- Services communicate using droplet IP address

## API Documentation
- OpenAPI schema available at `auth-service/api_schema.yml`
- View at: https://editor.swagger.io/

## Development Workflow

1. **Local Development**: Use `cargo watch` for hot reload during development
2. **Testing**: Run unit tests frequently, integration tests before commits  
3. **Docker Testing**: Use `docker compose up` to test service integration
4. **Production**: Push to main branch triggers automated CI/CD pipeline

## Key Dependencies

### app-service
- `axum` - Web framework
- `askama` - HTML templating
- `reqwest` - HTTP client for auth-service calls
- `axum-extra` - Cookie handling

### auth-service  
- `axum` - Web framework
- `uuid` - Unique identifiers
- `validator` - Email validation
- `async-trait` - Async trait support
- `fake` + `quickcheck` - Testing utilities
