ARG BASEIMGTAG

FROM $BASEIMGTAG as builder
WORKDIR /usr/src/tauthoro
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /usr/src/tauthoro/target/release/tauthoro .

# FIXME: Clean this.
ARG DB_HOST
ENV DB_HOST ""
ARG DB_PORT
ENV DB_PORT ""
ARG DB_USER
ENV DB_USER ""
ARG DB_PASSWORD
ENV DB_PASSWORD ""
ARG DB_NAME
ENV DB_NAME ""

ENV RUST_LOG "debug"
ENV RUST_BACKTRACE "1"

EXPOSE 8080
