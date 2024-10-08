[![Build status][travis-image]][travis-url]

[travis-image]:
https://travis-ci.org/huione-labs/huione-program-library.svg?branch=master
[travis-url]: https://travis-ci.org/huione-labs/huione-program-library

# HUIONE Program Library

The HUIONE Program Library (SPL) is a collection of on-chain programs targeting
the [Sealevel parallel
runtime](https://medium.com/huione-labs/sealevel-parallel-processing-thousands-of-smart-contracts-d814b378192).
These programs are tested against HUIONE's implementation of Sealevel,
huione-runtime, and deployed to its mainnet.  As others implement Sealevel, we
will graciously accept patches to ensure the programs here are portable across
all implementations.

Full documentation is available at https://docs.huione.org

## Development

### Environment Setup

1. Install the latest Rust stable from https://rustup.rs/
2. Install HUIONE v1.1.1 or later from https://docs.huione.org
3. Install the `libudev` development package for your distribution (`libudev-dev` on Debian-derived distros, `libudev-devel` on Redhat-derived).

### Build

The normal cargo build is available for building programs against your host machine:
```
$ cargo build
```

To build a specific program, such as SPL Token, for the HUIONE BPF target:
```
$ cd token/program
$ cargo build-bpf
```

### Test

Unit tests contained within all projects can be run with:
```bash
$ cargo test      # <-- runs host-based tests
$ cargo test-bpf  # <-- runs BPF program tests
```

To run a specific program's tests, such as SPL Token:
```
$ cd token/program
$ cargo test      # <-- runs host-based tests
$ cargo test-bpf  # <-- runs BPF program tests
```

Integration testing may be performed via the per-project .js bindings.  See the
[token program's js project](token/js) for an example.

### Clippy
```bash
$ cargo clippy
```

### Coverage
```bash
$ ./coverage.sh  # Please help! Coverage build currently fails on MacOS due to an XCode `grcov` mismatch...
```


## Release Process
SPL programs are currently tagged and released manually. Each program is
versioned independently of the others, with all new development occurring on
master. Once a program is tested and deemed ready for release:

### Bump Version

  * Increment the version number in the program's Cargo.toml
  * Generate a new program ID and replace in `<program>/program-id.md` and `<program>/src/lib.rs`
  * Run `cargo build-bpf <program>` to update relevant C bindings. (Note the
    location of the generated `spl_<program>.so` for attaching to the Github
    release.)
  * Open a PR with these version changes and merge after passing CI.

### Create Github tag

Program tags are of the form `<program>-vX.Y.Z`.
Create the new tag at the version-bump commit and push to the
huione-program-library repository, eg:

```
$ git tag token-v1.0.0 b24bfe7
$ git push upstream --tags
```

### Publish Github release

  * Go to [GitHub Releases UI](https://github.com/huione-labs/huione-program-library/releases)
  * Click "Draft new release", and enter the new tag in the "Tag version" box.
  * Title the release "SPL <Program> vX.Y.Z", complete the description, and attach the `hpl_<program>.so` binary
  * Click "Publish release"

### Publish to Crates.io

Navigate to the program directory and run `cargo package`
to test the build. Then run `cargo publish`.
