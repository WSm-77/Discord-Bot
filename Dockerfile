FROM rust:1.87-slim AS chef
RUN cargo install --locked cargo-chef
WORKDIR /app

# --- planner stage: resolve dependencies only -----------------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- builder stage: use cached deps then build real code -------
FROM chef AS builder

# Install cargo-shuttle
RUN cargo install cargo-shuttle

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

CMD ["cargo", "shuttle", "run", "--release"]
