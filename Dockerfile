FROM docker.io/library/rust:slim
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 8000
CMD [ "./target/release/doc" ]