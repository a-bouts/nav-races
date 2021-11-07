FROM debian as builder

ARG TARGETPLATFORM

COPY ./aarch64-unknown-linux-gnu /target/aarch64-unknown-linux-gnu

RUN ls -lR /target

RUN if [ $TARVETPLATFORM ="linux/arm64" ]; then \
    mv /target/aarch64-unknown-linux-gnu/release/races /races; \
  elif [ $TARVETPLATFORM ="linux/amd64" ]; then \
    mv x86_64-unknown-linux-gnu/release/races /races; \
  fi


FROM scratch

COPY --from=builder /races /

ENTRYPOINT ["/races"]
