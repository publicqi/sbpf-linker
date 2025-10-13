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
      --cpu <CPU>                       Target BPF processor. Can be one of `generic`, `probe`, `v1`, `v2`, `v3` [default: generic]
  -o, --output <OUTPUT>                 Write output to <output>
      --btf                             Emit BTF information
      --allow-bpf-trap                  Permit automatic insertion of __`bpf_trap` calls. See: <https://github.com/llvm/llvm-project/commit/ab391beb11f733b526b86f9df23734a34657d876>
  -L <LIBS>                             Add a directory to the library search path
  -O <OPTIMIZE>                         Optimization level. 0-3, s, or z [default: 2]
      --export-symbols <path>           Export the symbols specified in the file `path`. The symbols must be separated by new lines
      --unroll-loops                    Try hard to unroll loops. Useful when targeting kernels that don't support loops
      --ignore-inline-never             Ignore `noinline`/`#[inline(never)]`. Useful when targeting kernels that don't support function calls
      --dump-module <path>              Dump the final IR module to the given `path` before generating the code
      --llvm-args <args>                Extra command line arguments to pass to LLVM
      --disable-expand-memcpy-in-order  Disable passing --bpf-expand-memcpy-in-order to LLVM
      --disable-memory-builtins         Disable exporting `memcpy`, `memmove`, `memset`, `memcmp` and `bcmp`. Exporting those is commonly needed when LLVM does not manage to expand memory intrinsics to a sequence of loads and stores
      --export <symbols>                Comma separated list of symbols to export. See also `--export-symbols`
      --fatal-errors <FATAL_ERRORS>     Whether to treat LLVM errors as fatal [default: true] [possible values: true, false]
  -h, --help                            Print help
  -V, --version                         Print version
```


### Generate a Program

```sh
cargo generate --git https://github.com/blueshift-gg/solana-upstream-bpf-template
```

### Build

```sh
cargo build-bpf
```