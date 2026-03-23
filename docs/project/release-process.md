# Release Process

This is the practical release flow currently used for Uxarion.

## Source of truth

- canonical remote: `uxarion`
- canonical branch: `main`

Do not treat the old `origin` fork as the release target unless there is a deliberate reason.

## Version bump

Update both:

- `codex-rs/Cargo.toml`
- `codex-cli/package.json`

The npm package version and the runtime version should normally match.

## Build the runtime

Current practical build command:

```bash
cd /mnt/c/codex-hacker/codex-rs
cargo build -p codex-cli --bin codex
```

Notes:

- a proper release build is preferable
- in constrained environments, debug builds may still be used as a fallback, but strip the binary before packaging if size becomes unreasonable

## Package the runtime archive

The runtime archive lives under:

- `releases/vX.Y.Z/uxarion-X.Y.Z-linux-x64.tar.xz`

Current packaging pattern:

1. unpack the previous release archive
2. replace the bundled `codex` binary with the new build
3. repack under the new version directory

The archive should contain the expected `package/vendor/...` layout used by the npm wrapper.

## Verify before shipping

Minimum checks:

1. the binary reports the expected version
2. the packaged archive extracts successfully
3. the extracted runtime binary reports the expected version
4. `npm pack --dry-run` looks correct
5. a fresh npm install can download the runtime and run `--version`

Useful commands:

```bash
npm pack --dry-run
npm view uxarion version
```

## Push and publish

1. push source commits to `uxarion/main`
2. create the GitHub release and attach the runtime archive
3. publish npm from `codex-cli/`

Typical commands:

```bash
git push uxarion HEAD:main
gh release create vX.Y.Z releases/vX.Y.Z/uxarion-X.Y.Z-linux-x64.tar.xz --repo rachidlaad/uxarion --target main
cd codex-cli && npm publish
```

## Public verification

After publishing:

1. verify the GitHub release exists
2. verify the raw runtime URL returns `200`
3. verify `npm view uxarion version`
4. verify a fresh npm install downloads the runtime and runs successfully

## Release notes

Keep release notes short:

- what changed
- what was fixed
- current platform scope if still limited

## Known release caveats

- mounted Windows workspaces can make builds, tests, and clippy runs much slower
- full `codex-tui` validation may be impractical there; call out any partial-validation release explicitly
