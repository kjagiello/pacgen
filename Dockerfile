FROM clux/muslrust AS builder

# Rust lacks a straightforward way to only install dependencies, so we have to fake the existence
# of the project in order for this to work. The idea is basically to build a separate layer with
# only the dependencies, so that we don't have to reinstall them on every source code change.
# Related issue: https://github.com/rust-lang/cargo/issues/2644
RUN mkdir src && touch src/lib.rs && echo "fn main() {}" > src/cli.rs
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN cargo build --release

# Build the source code without installing the dependencies. In order to make rust pick up changes
# in the source files, we have to bump their "date modified".
COPY ./src/ ./src/
RUN find src/ -type f -exec touch {} + && cargo build --release
RUN strip ./target/x86_64-unknown-linux-musl/release/pacgen

FROM scratch AS bin
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/pacgen /

EXPOSE 8080
ENTRYPOINT ["/pacgen"]
