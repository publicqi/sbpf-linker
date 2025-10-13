<h1 align="center">
  SBPF Linker
</h1>
<p align="center">
  An upstream BPF linker to relink upstream BPF binaries into an SBPF V0 compatible binary format.
</p>

### Install

```sh
cargo install sbpf-linker
```

### Usage

```
Usage: sbpf-linker [OPTIONS] --output <OUTPUT> <INPUTS>...

Arguments:
  <INPUTS>...  Input files. Can be object files or static libraries

Options:
      --cpu <CPU>              Target BPF processor. Can be one of `generic`, `probe`, `v1`, `v2`, `v3` [default: generic]
  -o, --output <OUTPUT>        Write output to <output>
      --btf                    Emit BTF information
  -L <LIBS>                    Add a directory to the library search path
  -O <OPTIMIZE>                Optimization level. 0-3, s, or z [default: 2]
      --export-symbols <path>  Export the symbols specified in the file `path`. The symbols must be separated by new lines
      --dump-module <path>     Dump the final IR module to the given `path` before generating the code
      --export <symbols>       Comma separated list of symbols to export. See also `--export-symbols`
  -h, --help                   Print help
  -V, --version                Print version
```


### Generate a Program

```sh
cargo generate --git https://github.com/blueshift-gg/solana-upstream-bpf-template
```

### Build

```sh
cargo build-bpf
```