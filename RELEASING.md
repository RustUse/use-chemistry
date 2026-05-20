# Releasing

This workspace supports manual, safety-first publish flows. It does not
publish automatically on normal pushes.

## crates.io token setup

1. Create or reuse a crates.io API token with publish access for the intended
   chemistry crates.
2. Add the token to the GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Do not print the token in logs or local shell history.

## GitHub Actions secret

- Secret name: `CARGO_REGISTRY_TOKEN`

## Dry-run publish

Use the `Publish` workflow with:

- `crate = all` or one specific workspace crate
- `dry_run = true`

This is the default mode and is the safest way to validate publish packaging
before a real release.

## Manual publish

Use the `Publish` workflow with:

- `crate = all` to publish the full workspace in dependency order, or one
  specific crate
- `dry_run = false`

The workflow runs formatting, linting, tests, and `cargo check` before it
attempts any publish step.

## Initial publish order

For the first public crates.io wave, publish in dependency order:

1. `use-element`
2. `use-isotope`
3. `use-chemical-formula`
4. `use-compound`
5. `use-molecule`
6. `use-atomic-number`
7. `use-atomic-mass`
8. `use-electron-shell`
9. `use-periodic-table`
10. `use-chemistry`

`use-element` must exist on crates.io before the other chemistry crates can be
published. `use-compound` and `use-molecule` must wait until
`use-chemical-formula` is visible on crates.io. The umbrella crate
`use-chemistry` should come last after the focused crates are visible on
crates.io.

## Post-initial-release automation

After the first manual crates.io release wave for the `use-chemistry`
workspace, the repository can use the `release-plz` workflows for follow-up
releases.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request based on
  `release-plz.toml` and the current changelog rules

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: manual dispatch only
- Required input: `post-initial-release = true`
- Purpose: confirms all published chemistry crates already exist on crates.io,
  then runs `release-plz release`

Real release-plz publishes still require `CARGO_REGISTRY_TOKEN` unless the
repository later moves to trusted publishing.

## Local dry-run examples

```sh
cargo publish -p use-element --dry-run
cargo publish -p use-isotope --dry-run
cargo publish -p use-chemical-formula --dry-run
cargo publish -p use-compound --dry-run
cargo publish -p use-molecule --dry-run
cargo publish -p use-periodic-table --dry-run
```

## Semver notes

- Patch bumps are for compatible fixes and small additive maintenance changes.
- Minor bumps are for additive API changes during `0.x` development.
- Major bumps are for breaking changes once the crates stabilize at `1.0.0`.
- Pre-release identifiers should remain intentional and explicit.

## Permanent version warning

Published crates.io versions are permanent. You cannot replace an already
published version with new contents, so verify the crate list, metadata, and
changelog inputs before any real publish.
