####################################################################################################
## Build and serve
####################################################################################################
FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
#RUN update-ca-certificates

#ENV USER=server
#ENV UID=10001
#
#RUN adduser \
#    --disabled-password \
#    --gecos "" \
#    --home "/nonexistent" \
#    --shell "/sbin/nologin" \
#    --no-create-home \
#    --uid "${UID}" \
#    "${USER}"

WORKDIR /client

COPY ./ .


RUN trunk serve --release
