# RustMailer

RustMailer is a Rust-based web application for handling and sending emails, built from scratch.

## Requirements

- Rust (latest stable version)
- Cargo
- PostgreSQL
- Docker (optional, for containerized setup)

## Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rustmailer
   ```

2. Configure the database:
   Ensure PostgreSQL is running and create a new database for RustMailer.

3. Set environment variables:
   Create a `.env` file in the project root with the necessary configuration such as database URL, application port, and email server credentials.

4. Build and run the application:
   ```bash
   cargo run
   ```

## Testing

Run the test suite with:

```bash
cargo test
```

This will execute all unit and integration tests for RustMailer.
