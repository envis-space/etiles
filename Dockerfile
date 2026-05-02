FROM rust:1.95.0-trixie AS builder

COPY . /home/app
WORKDIR /home/app

RUN apt-get update && apt-get install -y cmake sqlite3 libtiff-dev # installations for building proj-sys

# prepare libs
RUN mkdir -p /home/libs
RUN cp "$(ldconfig -p | grep libz.so.1 | tr ' ' '\n' | grep /)" /home/libs
RUN cp "$(ldconfig -p | grep libsqlite3.so.0 | tr ' ' '\n' | grep /)" /home/libs

# build application
RUN cargo build --profile release-lto


FROM gcr.io/distroless/cc-debian13 AS runtime

WORKDIR /app

# zstandard lib
COPY --from=builder /home/libs/libz.so.1 /lib/libz.so.1
# proj lib
COPY --from=builder /home/libs/libsqlite3.so.0 /lib/libsqlite3.so.0
COPY --from=builder /home/app/target/release-lto/build/proj-sys-*/out/share/proj /app/share/proj
ENV PROJ_DATA=/app/share/proj

COPY --from=builder /home/app/target/release-lto/etiles-cli /app/app

ENTRYPOINT ["/app/app"]
