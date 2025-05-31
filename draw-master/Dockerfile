FROM rust:1.87-slim

WORKDIR /app

# Install required dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Install cargo-shuttle
RUN cargo install cargo-shuttle

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Secrets.toml ./
COPY .shuttle ./shuttle

# Build the project
RUN cargo build --release

# Run the shuttle service
CMD ["cargo", "shuttle", "run"]
