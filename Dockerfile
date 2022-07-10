FROM rust:latest as builder

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6666

WORKDIR /app
COPY . .

RUN ls -la /usr/local/cargo/bin/
RUN rustup default nightly
RUN cargo install --path .
RUN ls -la /usr/local/cargo/bin/

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/rocket-api /usr/local/bin/rocket-app
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 6666
CMD ["rocket-app"]