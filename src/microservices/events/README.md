# Proxy

There is the simplest REST API application of custom proxy for cinemaabyss project. 

## Installation

### Local launching without docker
1. Clone the repository
2. Run `cargo run --bin proxy` to build project

### Local launching with docker
1. Cone the repository
2. Run `docker build -t proxy:latest .`
3. Run `docker run --name proxy-service -p 8000:8000 proxy:latest`
