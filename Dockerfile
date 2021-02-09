# --- Use custom rust-musl-builder image
FROM ekidd/rust-musl-builder as BUILDER

# Add the Rust-Lang source-code
ADD --chown=rust:rust . ./

# Build the application
RUN cargo build --release

# --- Build the deployment container
FROM gliderlabs/alpine:latest

# Install Alpine dependencies
RUN apk upgrade --update-cache --available && \
    rm -rf /var/cache/apk/*

# Copy compiled musl binary
COPY --from=BUILDER \
    ./home/rust/src/target/x86_64-unknown-linux-musl/release/blue-green-demo \
    ./

# --- Copy files
COPY . .

# --- Run Rust application
CMD /blue-green-demo
