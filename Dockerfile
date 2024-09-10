# We use the latest Rust stable release as base image
FROM rust:slim

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .

ENV SQLX_OFFLINE true

RUN apt-get install pkg-config libssl-dev -y

# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release

ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/forge"]
