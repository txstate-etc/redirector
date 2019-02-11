FROM clux/muslrust:nightly AS build

COPY . /root/
WORKDIR /root/
RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cd health \
  && cargo build --release --target x86_64-unknown-linux-musl \
  && cd ../ \
  # Utilize RHEL apache2 uid/gid for redirector www user
  && groupadd -r -g 48 www \
  && useradd -r -u 48 -g 48 -c 'Redirector Service' -d /var/lib/www www \
  && mkdir -p /rootfs/etc/ /rootfs/bin/ /rootfs/var/lib/www/ \
  && chown -R www. /rootfs/var/lib/www/ \
  && cp /etc/passwd /etc/group /rootfs/etc/ \
  && ls -R /root/target/ \
  && cp /root/target/x86_64-unknown-linux-musl/release/redirector /rootfs/bin/ \
  && cp /root/health/target/x86_64-unknown-linux-musl/release/health /rootfs/bin/

FROM scratch AS target
COPY --from=build /rootfs/  /
USER www
WORKDIR /var/lib/www/
HEALTHCHECK CMD ["/bin/health"]
CMD ["/bin/redirector"]
