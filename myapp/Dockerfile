FROM rust:1.40-alpine as builder
WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/src/myapp/target/release/myapp .
CMD ["./myapp"]
