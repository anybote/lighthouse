FROM rust:1-alpine3.19 AS builder
COPY . /tmp/lighthouse
WORKDIR /tmp/lighthouse
RUN apk add musl-dev
RUN cargo build --release

FROM scratch AS runner
COPY --from=builder /tmp/lighthouse/assets/styles /assets/styles
COPY --from=builder /tmp/lighthouse/target/release/lighthouse /
COPY --from=builder /tmp/lighthouse/templates /templates
CMD ["/lighthouse"]
