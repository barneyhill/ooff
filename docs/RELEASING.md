# CI and releases

GitHub Actions runs formatting, Clippy, all Rust tests, package verification,
release builds and packaged-binary smoke tests on Linux x86_64/ARM64 and macOS
Intel/Apple Silicon. Linux also runs the independent benchmark verifier and
real-subprocess timeout tests. Workflows run on main pushes and pull requests;
CI can also be started manually. Oracle tests use optimization level 1 while
retaining debug assertions. Runners are GitHub-hosted, with 30-minute job limits.

The release workflow starts when a `v*` tag is pushed. It checks that the tag
matches Cargo.toml, repeats the complete CI matrix, and then publishes the source
crate using the repository Actions secret `CRATES_IO_API`. The token is exposed
only to the publishing step. No token belongs in a file or Cargo login command.

After checks pass, the workflow creates a GitHub release containing four binary
archives and SHA256SUMS, then uploads the crate to crates.io. A registry-upload
failure does not hide the successfully built binaries. Registry error details
are emitted as a check annotation, with the token explicitly redacted. Each archive has
`oofft`, `oofft-index`, `oofft-ddg`, usage documentation, the reference-relocation
script, and MIT/ViennaRNA parameter notices. macOS archives bundle the OpenMP runtime
and its license with relative library paths and ad-hoc signatures. They are not
Apple-notarized. Linux builds target Ubuntu 22.04 (glibc 2.35) and require libgomp1.
CPU-specific `target-cpu=native` is overridden by `target-cpu=generic` in CI.

For each release, update Cargo.toml and Cargo.lock, commit and push those changes,
then wait for main-branch CI to pass. Derive the tag from the committed version:

```sh
set -eu
version=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')
git tag "v$version"
git push origin "v$version"
```

Subsequent releases must update Cargo.toml/Cargo.lock before tagging. Published
crate versions cannot be overwritten. If crates.io fails, fix the reported registry error and rerun the failed job.
The GitHub upload is safe to retry for the same tag; do not attempt to republish
a crate version that already exists. The `Retry release publication` workflow
can also retry a failed publisher by run ID. It requires all build checks to have
passed and the release tag to still identify the tested commit. Verify the
crates.io account email before publishing.

The package include list excludes large benchmark artifacts and the local Cargo
CPU configuration. Benchmark records and source snapshots remain in Git and on
retained EC2 disks; they are not shipped to crates.io.
