FROM rust:buster as build

RUN cargo install cargo-watch

WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
    # Create a dummy file just so we can build deps
RUN touch dummy.rs &&\
    echo "fn main() {}" >> dummy.rs && \
    # change reference to main in the cargo file to point to our dummy file
    sed -i 's#src/main.rs#dummy.rs#' Cargo.toml && \
    # build deps
    cargo build --release && \
    # revert the dummy file reference back to the original
    sed -i 's#dummy.rs#src/main.rs#' Cargo.toml && \
    # remove the dummy now that we're done with it
    rm dummy.rs

COPY . .

RUN cargo build --release


FROM debian:buster

RUN apt-get update
RUN apt-get install libfontconfig -y

COPY --from=build /app/target/release/creative-coding /app/creative-coding

EXPOSE 8080

CMD ["/app/creative-coding"]