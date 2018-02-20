FROM rust:1.23.0

WORKDIR /usr/src/guessing_game
COPY . .

RUN cargo install

CMD ["guessing_game"]