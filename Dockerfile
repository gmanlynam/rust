FROM rust:1.23.0

WORKDIR /usr/src/mars_rover
COPY . .

RUN cargo install

CMD ["mars_rover"]