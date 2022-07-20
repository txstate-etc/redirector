FROM rust:1.62 AS build

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get -y install musl-dev musl-tools \
  && rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# docker build cache will be used to skip these slow steps.
WORKDIR /usr/src
RUN USER=root cargo new redirector
WORKDIR /usr/src/redirector
RUN USER=root cargo new health
COPY Cargo.* ./
COPY health/Cargo.* ./health/
RUN cargo build --release \
  && cd health \
  && cargo build --release

# Copy the source and build the application and health checker
WORKDIR /usr/src/redirector
COPY src ./src
COPY health/src ./health/src
RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cd health \
  && cargo build --release --target x86_64-unknown-linux-musl \
  && mkdir -p /rootfs/bin/ /rootfs/var/lib/www \
  # Utilize RHEL apache2 uid/gid for redirector www user
  && chown 48:48 /rootfs/var/lib/www \
  && cp /usr/src/redirector/target/x86_64-unknown-linux-musl/release/redirector /rootfs/bin/ \
  && cp /usr/src/redirector/health/target/x86_64-unknown-linux-musl/release/health /rootfs/bin/

FROM scratch
COPY --from=build /rootfs/  /
USER 48:48
WORKDIR /var/lib/www/
HEALTHCHECK CMD ["/bin/health"]
CMD ["/bin/redirector"]
