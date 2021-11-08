FROM debian as builder

ARG TARGETPLATFORM

COPY ./aarch64-unknown-linux-musl /target/aarch64-unknown-linux-musl

RUN ls -lR /target

RUN if [ $TARGETPLATFORM = "linux/arm64" ]; then \
    mv /target/aarch64-unknown-linux-musl/release/races /races; \
  elif [ $TARGETPLATFORM = "linux/amd64" ]; then \
    mv x86_64-unknown-linux-musl/release/races /races; \
  fi; \
  chmod +x /races


FROM debian

COPY --from=builder /races /

CMD ["/races"]
