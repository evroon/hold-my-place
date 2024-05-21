# Build Stage
FROM rust:latest as builder
RUN USER=root cargo new --bin hold-my-place
WORKDIR /hold-my-place
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release \
    && rm src/*.rs
ADD . ./
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim AS final

ARG APP=/usr/src/app
ENV APP_USER=holdmyplace
RUN rm -rf /var/lib/apt/lists/* \
    && groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /hold-my-place/target/release/hold-my-place ${APP}/hold-my-place
COPY --from=builder /hold-my-place/assets ${APP}/assets

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

EXPOSE 3300
CMD ["./hold-my-place"]
