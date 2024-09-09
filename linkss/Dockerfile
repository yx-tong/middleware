FROM python:3.7.4-stretch
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.bashrc
WORKDIR /app
RUN /root/.cargo/bin/cargo new --bin rust-docker-web
WORKDIR /app/rust-docker-web
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN /root/.cargo/bin/cargo build --release
RUN rm src/*.rs
RUN rm -rf target/release
COPY ./src ./src
RUN /root/.cargo/bin/cargo build --release