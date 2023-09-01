FROM ubuntu AS builder
# Use apk for package management in Alpine
RUN apt update
RUN apt install -y wget tar
WORKDIR /app
RUN wget https://github.com/paritytech/substrate-contracts-node/releases/download/v0.30.0/substrate-contracts-node-linux.tar.gz
RUN tar -xzvf substrate-contracts-node-linux.tar.gz

FROM ubuntu AS runtime
COPY --from=builder /app/artifacts/substrate-contracts-node-linux/substrate-contracts-node /usr/local/bin
ENTRYPOINT [ "substrate-contracts-node", "--rpc-external" ]
