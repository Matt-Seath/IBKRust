FROM rust:alpine as prepare-stage
WORKDIR /app
COPY . .

FROM prepare-stage as build-stage
RUN cargo build --release

FROM rust:alpine
EXPOSE 3000
CMD [ "ibkrust" ]


