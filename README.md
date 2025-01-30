# rustrandserve

**rustrandserve** is a lightweight server written in Rust that serves files from a `data` directory or returns random content for each request. It helps obscure files (like `cat`, `dog`, etc.) to make web scans (e.g., with Gobuster) more challenging.

## Features

- Serve files dynamically from the `data` directory.
- Return random content if the requested file is not found.
- Makes web scanning tools like Gobuster harder to use.

## Installation

### Prerequisites

- Install [Rust](https://www.rust-lang.org/learn/get-started) if you haven't already.

### Steps

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/rustrandserve.git
    cd rustrandserve
    ```

2. Build and run the project:

    ```bash
    cargo run
    ```

The server will be available at `http://127.0.0.1:9999`.

## Usage

1. Create a `data` folder and add files (e.g., `cat`, `dog`, etc.).
2. By default, the port is `9999`, but can be specified with the execution, for exemple:
    ```bash
    rustrandserve 1234
    ```
3. Run the server, then visit the following URLs:

    - `http://127.0.0.1:1234/cat` – Returns the content of the `cat` file.
    - `http://127.0.0.1:1234/randomfile` – Returns a random string.
    - `http://127.0.0.1:1234/` – Returns a random string.

## Docker

The `Dockerfile` is in the repo | [Dockerfile](Dockerfile)
To build:
```bash
docker build -t rustrandserve .
```

Here the `docker-compose.yml`:
```yml
services:
    nginx:
        image: rustrandserve
        container_name: rustrandserve
        volumes:
            - ./data:/app/data:ro
        ports:
            - "9999:9999"
        restart: unless-stopped
```

## Obfuscation with Gobuster

To make file discovery harder, use a web scanner like [Gobuster](https://github.com/OJ/gobuster):

```bash
gobuster dir -u http://127.0.0.1:9999 -w /path/to/wordlist.txt
