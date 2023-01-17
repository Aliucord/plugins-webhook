FROM --platform=$BUILDPLATFORM rust AS BUILD
WORKDIR /app/

# Setup target arch
ARG TARGETARCH
RUN rustup target add $(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/')-unknown-linux-musl

# Setup musl cross compiler
# https://musl.cc/aarch64-linux-musl-cross.tgz & https://musl.cc/x86_64-linux-musl-cross.tgz
RUN cd /tmp && \
    curl -L https://github.com/xmake-mirror/musl.cc/releases/download/20210202/aarch64-linux-musl-cross.linux.tgz -o aarch64-linux-musl-cross.tgz && \
    curl -L https://github.com/xmake-mirror/musl.cc/releases/download/20210202/x86_64-linux-musl-cross.linux.tgz -o x86_64-linux-musl-cross.tgz && \
    tar -xzf aarch64-linux-musl-cross.tgz && \
    tar -xzf x86_64-linux-musl-cross.tgz
ENV CC_aarch64_unknown_linux_musl=/tmp/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CC_x86_64_unknown_linux_musl=/tmp/x86_64-linux-musl-cross/bin/x86_64-linux-musl-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

# Fetch cargo dependencies
ADD Cargo.toml Cargo.lock ./
RUN cargo fetch

ADD src ./src

RUN cargo build --release --target $(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/')-unknown-linux-musl
RUN mv /app/target/$(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/')-unknown-linux-musl/release/plugins-webhook .

FROM --platform=$TARGETPLATFORM alpine
WORKDIR /app
EXPOSE 8000
COPY --from=BUILD /app/plugins-webhook .
CMD ["/app/plugins-webhook"]
