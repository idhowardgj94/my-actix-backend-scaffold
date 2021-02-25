FROM rust:1.49

WORKDIR /usr/src/myapp
COPY . . 

RUN cargo install --path .

CMD ["myapp"]

