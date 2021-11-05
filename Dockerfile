########################## BUILD IMAGE  ##########################
FROM --platform=$BUILDPLATFORM rust:1.50 as build

ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Build time options to avoid dpkg warnings and help with reproducible builds.
ENV DEBIAN_FRONTEND=noninteractive LANG=C.UTF-8 TZ=UTC TERM=xterm-256color

# Don't download rust docs
RUN rustup set profile minimal

ENV CARGO_HOME "/root/.cargo"
ENV USER "root"

# Creates a dummy project used to grab dependencies
RUN USER=root cargo new --bin /app
WORKDIR /app

# Copies over *only* your manifests and build files
COPY ./Cargo.* ./
COPY ./rust-toolchain ./rust-toolchain
COPY ./build.rs ./build.rs

#RUN rustup target add aarch64-unknown-linux-musl

# Builds your dependencies and removes the
# dummy project, except the target folder
# This folder contains the compiled dependencies
#RUN cargo build --release --target=aarch64-unknown-linux-musl
RUN cargo build --release
RUN find . -not -path "./target*" -delete

# Copies the complete project
# To avoid copying unneeded files, use .dockerignore
COPY . .

# Make sure that we actually build the project
RUN touch src/main.rs

# Builds again, this time it'll just be
# your actual source files being built
#RUN cargo build --release --target=aarch64-unknown-linux-musl
RUN cargo build --release


######################## RUNTIME IMAGE  ########################
# Create a new stage with a minimal image
# because we already have a binary built
FROM debian

ENV ROCKET_ENV "production"
ENV ROCKET_ADDRESS "0.0.0.0"
ENV ROCKET_PORT=80
ENV ROCKET_WORKERS=10

EXPOSE 80

WORKDIR /

# Copies the files from the context (Rocket.toml file and web-vault)
# and the binary from the "build" stage to the current stage
COPY --from=build /app/target/release/races .

# Configures the startup!
CMD ["/races"]
