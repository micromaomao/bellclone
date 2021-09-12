FROM rust:latest as build
WORKDIR /tmp/flatbuffers
RUN apt-get update && apt-get install curl unzip && \
    curl -sL 'https://github.com/google/flatbuffers/releases/download/v2.0.0/Linux.flatc.binary.clang++-9.zip' -o flatc.zip && \
    unzip flatc.zip && \
    cp flatc /usr/bin && \
    chmod +x /usr/bin/flatc
WORKDIR /usr/src/app
COPY . .
RUN cd server && cargo build --release

FROM debian:latest
COPY --from=build --chown=0:0 /usr/src/app/server/target/release/server /server
RUN chmod a+rx /server
USER 1000:1000
ENTRYPOINT [ "/server" ]
CMD [ "0.0.0.0:5000" ]
EXPOSE 5000
