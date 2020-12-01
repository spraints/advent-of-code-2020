# rustlang/rust:nightly on 2020-12-01
FROM rustlang/rust@sha256:56ad60384b8e47c6693eb2e4b7844d08adc5812d9ab812a84bed296351cdc898

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 3001

VOLUME ["/usr/local/cargo"]
