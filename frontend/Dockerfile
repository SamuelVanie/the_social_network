FROM "debian"

# Update and install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsoup2.4-dev \
    libwebkit2gtk-4.0-dev

# Install Rust 1.70
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.70.0
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Node.js 21.6.2
RUN curl -sL https://deb.nodesource.com/setup_21.x | bash -
RUN apt-get install -y nodejs

WORKDIR /app

# Copy the source code from parent directory to the working directory
COPY . .

# Install tauri dependencies
RUN npm install @tauri-apps/cli@latest @tauri-apps/api@latest
RUN npm install tauri

# Build the application for release purpose
RUN cargo build --release --manifest-path=./src-tauri/Cargo.toml --no-default-features

# Run the application for release purpose
# CMD ["./src-tauri/target/release/app"]

# Run the application for development purpose
CMD ["cargo", "tauri", "dev"]
