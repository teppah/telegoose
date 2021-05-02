FROM rust:1.51

WORKDIR /usr/src/telegoose
COPY . .

RUN cargo install --path .
EXPOSE 443
CMD ["telegoose"]