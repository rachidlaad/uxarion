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

Important:

- the user-facing Uxarion release version may diverge from the OpenAI compatibility version sent to ChatGPT/Codex backend endpoints
- do not overwrite backend compatibility headers with the public release version unless the backend model catalog has been verified against that client version

## Build the runtime

For public releases, prefer GitHub Actions instead of building locally.

The `npm-publish` workflow now handles the Linux runtime asset for a release:

- resolves `codex-cli/package.json`
- builds `codex-cli` on GitHub when the expected runtime asset is missing
- packages `package/vendor/x86_64-unknown-linux-musl/codex/codex`
- uploads `uxarion-X.Y.Z-linux-x64.tar.xz` to the matching GitHub release
- publishes the npm wrapper after the runtime URL is reachable

Local fallback build command:

```bash
cd /mnt/c/codex-hacker/codex-rs
cargo build -p codex-cli --bin codex
```

Notes:

- a proper release build is preferable
- in constrained environments, debug builds may still be used as a fallback, but strip the binary before packaging if size becomes unreasonable

## Package the runtime archive

The runtime archive is cut locally as:

- `releases/vX.Y.Z/uxarion-X.Y.Z-linux-x64.tar.xz`

Current packaging pattern:

1. unpack the previous release archive
2. replace the bundled `codex` binary with the new build
3. repack under the new version directory

The archive should contain the expected `package/vendor/...` layout used by the npm wrapper.

The canonical public runtime path is the GitHub Release asset:

- `https://github.com/rachidlaad/uxarion/releases/download/vX.Y.Z/uxarion-X.Y.Z-linux-x64.tar.xz`

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
2. create and publish the GitHub release tag, for example `vX.Y.Z`
3. let the GitHub Actions npm publish workflow build/upload the Linux runtime asset if it is missing
4. let the same workflow publish the wrapper package from the tagged source

Required repo secret:

- `NPM_TOKEN`

Typical commands:

```bash
git push uxarion HEAD:main
gh release create vX.Y.Z --repo rachidlaad/uxarion --target main --title vX.Y.Z --notes "Uxarion X.Y.Z"
```

The npm publish workflow should be triggered by the published GitHub release. If needed, rerun it manually with `workflow_dispatch` from the GitHub Actions UI.

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
