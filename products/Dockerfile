FROM rust:1.19.0

WORKDIR /usr/src/products
COPY . .

RUN cargo install

CMD ["cargo", "run"]