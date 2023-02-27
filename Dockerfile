# ------------------ Chef stage -------------------
# Use cargo chef to cache dependencies
FROM rustlang/rust:nightly AS chef

# Install cargo chef
RUN cargo install cargo-chef 

# Work in app
WORKDIR /app

# ------------------ Planner stage -------------------
FROM chef as planner
# Copy files into container
COPY . .

# Create a lockfile for cargo chef
RUN cargo +nightly chef prepare --recipe-path recipe.json


# ------------------ Builder stage -------------------
FROM chef AS builder

# Copy over our lock file
COPY --from=planner  /app/recipe.json recipe.json

# Build dependencies - not the app
RUN cargo chef cook --release --recipe-path recipe.json

### Above this all dependencies should be cached as long as our lock file stays the same

COPY . .

# Build binary
RUN cargo build --release

# ------------------ Runtime stage -------------------

# Using super lightweight debian image to reduce overhead
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# Copy prebuild bin from the Builder stage
COPY --from=builder /app/target/release/huffc /usr/local/bin/huffc

# Run bin which has been copied from the builder stage :)
ENTRYPOINT [ "/bin/sh", "-c" ]