# Contributing

Thanks for contributing to RustUse/use-chemistry.

## Before opening a pull request

- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo check --workspace --all-features --examples` when examples exist.
- Run `cargo deny check` when `cargo-deny` is installed locally.

## Repository model

- GitHub is the canonical repository and release authority.
- Public mirrors may accept patches, but release tags and crates.io publishes are cut from GitHub.
- Preserve authorship and provenance when porting changes from mirrors.

## Release flow

- Keep changelog-impacting changes clear in pull requests.
- Follow [RELEASING.md](RELEASING.md) for publish order and release automation.
- Do not change crate names, version policy, or publish sequencing without maintainer approval.

## Security and conduct

- Report vulnerabilities privately as described in the
  [RustUse security policy](https://github.com/RustUse/.github/blob/main/SECURITY.md).
- Report conduct issues privately as described in the
  [RustUse code of conduct](https://github.com/RustUse/.github/blob/main/CODE_OF_CONDUCT.md).
