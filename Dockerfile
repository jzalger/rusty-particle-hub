FROM rust:latest as builder
WORKDIR /usr/src/particlehub
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/particlehub /usr/local/bin/particlehub
ENV ROCKET_ENV=development
EXPOSE 7070
CMD [ "/usr/local/bin/particlehub" ]
