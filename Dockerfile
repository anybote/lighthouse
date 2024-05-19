FROM rust:1-alpine3.19 AS builder
COPY . /tmp/lighthouse
WORKDIR /tmp/lighthouse
RUN apk add musl-dev
RUN cargo build --release

FROM rust:1-alpine3.19 AS runner
COPY --from=builder /tmp/lighthouse/assets/styles /opt/lighthouse/assets/styles
COPY --from=builder /tmp/lighthouse/target/release/lighthouse /opt/lighthouse/lighthouse
COPY --from=builder /tmp/lighthouse/templates /opt/lighthouse/templates
ENTRYPOINT cd /opt/lighthouse && ./lighthouse
