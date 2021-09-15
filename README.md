# Moon Rabbit

Implementation of a https://moonrabbit.com node.

## Building

If you'd like to build from source, first install Rust. You may need to add Cargo's bin directory
to your PATH environment variable. Restarting your computer will do this for you automatically.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software:

```bash
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
git checkout <latest tagged release>
./scripts/init.sh
cargo build --release
```

Note that compilation is a memory intensive process. We recommend having 4 GiB of physical RAM or swap available (keep in mind that if a build hits swap it tends to be very slow).

## Contributing

### Contributing Guidelines

[Contribution Guidelines](CONTRIBUTING.md)

### Contributor Code of Conduct

[Code of Conduct](CODE_OF_CONDUCT.md)

## License

[GPL 3.0 licensed](LICENSE).
