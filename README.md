# Emma - Lightweight Embeddable Matrix Server

Emma is a lightweight, embeddable instance of a Matrix server that is designed to run under Matrix. This project is written in Rust and uses the WAGI interface to interact with clients.

It is expected that, in the future, Emma runs orchestrated by the [Valor][-1] runtime.

## Folder Structure

```bash
- core/            # A library that contains the abstractions of Module, Request, Response that modules use to interact with the client
- client/          # A WASI client that includes the existing modules and exposes a Router based on WAGI
- modules/
  |- well-known/   # A module that exposes /.well-known/matrix/client
```

- The `core/` folder contains the core library of the project, which includes the abstractions for modules, requests, and responses.
- The `client/` folder contains the WASI client that includes the existing modules and exposes a router based on WAGI.
- The `modules/` folder contains the individual modules, with the `well-known/` module being the only one currently available, which exposes the `/.well-known/matrix/client` endpoint.

## Dependencies

- Rust: The project is written in Rust, so it needs to be installed first.
- WAGI: Emma uses WAGI to run, so it is a required dependency.
- Other dependencies are listed in the` Cargo.toml` file on each project.

## Installation

1. Install Rust: Emma is written in Rust, so you will need to install Rust first. You can download Rust from [here][0].
2. Install `cargo-wasi`: Emma uses `cargo-wasi` to build WASI-compatible binaries. You can install cargo-wasi using the following command:

```bash
cargo install cargo-wasi
```

3. WAGI: Emma uses WAGI to run, so it is a required dependency. You can install WAGI by following the [instructions][1].
4. Clone the repository: Clone the Emma repository using Git by running the following command:

```bash
git clone https://github.com/virto-network/emma.git
```

5. Build the project: Navigate to the cloned repository and run the following command to build the project:

```bash
cargo wasi build --release
```

## Usage

1. Run Emma: After building the project, you can run Emma by running the following command:

```bash
wagi --config client/config.toml
```

2. Access Emma: Once Emma is running, you can access it by visiting `http://localhost:3000` in your web browser.

## Testing

Emma comes with a test suite that can be run using the following command:

```bash
cargo test
```

This will run all the tests in the project and provide a summary of the results.

[-1]: https://github.com/virto-network/valor
[0]: https://www.rust-lang.org/tools/install
[1]: https://github.com/deislabs/wagi
