# SBPF Linker

An upstream BPF linker to relink upstream BPF binaries into an SBPF V0 compatible binary format.

### Usage
Install with:
```sh
cargo install sbpf-linker
```

Create a new program template with
```sh
cargo generate --git https://github.com/blueshift-gg/solana-upstream-bpf-template
```

Build program with:
```sh
cargo build-bpf
```