####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder
RUN update-ca-certificates

# Create appuser
ENV USER=server
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /server

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /server

# Copy our build
COPY --from=builder /server/target/release ./

# Use an unprivileged user.
USER server:server

CMD ["/server/server"]