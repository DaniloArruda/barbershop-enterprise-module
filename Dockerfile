# Build stage
FROM rust:buster as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL

# Make sure to use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

RUN cargo build || true

# RUN apt install build-essentials libssl-dev -y
RUN apt-get update && apt-get install -y build-essential libssl-dev cmake

# Copy the source code
COPY . .
COPY local_development local_development

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/enterprise_module .

CMD ["./enterprise_module"]
