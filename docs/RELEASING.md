# CI and releases

GitHub Actions runs formatting, Clippy, all Rust tests, package verification,
release builds and packaged-binary smoke tests on Linux x86_64/ARM64 and macOS
Intel/Apple Silicon. Linux also runs the independent benchmark verifier and
real-subprocess timeout tests. Workflows run on main pushes and pull requests;
CI can also be started manually. Runners are GitHub-hosted, with 30-minute job limits.

The release workflow starts when a `v*` tag is pushed. It checks that the tag
matches Cargo.toml, repeats the complete CI matrix, and then publishes the source
crate using the repository Actions secret `CRATES_IO_API`. The token is exposed
only to the publishing step. No token belongs in a file or Cargo login command.

After checks pass, the workflow uploads the crate to crates.io and creates a
GitHub release containing four binary archives and SHA256SUMS. Each archive has
`ooff`, `ooff-index`, README and license. macOS archives bundle the OpenMP runtime
and its license with relative library paths and ad-hoc signatures. They are not
Apple-notarized. Linux builds target Ubuntu 22.04 (glibc 2.35) and require libgomp1.
CPU-specific `target-cpu=native` is overridden by `target-cpu=generic` in CI.

For the first release, after the main-branch CI is green:

```sh
git tag v0.1.0
git push origin v0.1.0
```

Subsequent releases must update Cargo.toml/Cargo.lock before tagging. Published
crate versions cannot be overwritten. If crates.io succeeds but GitHub release
creation fails, upload the retained Actions artifacts to that tag's GitHub
release; do not attempt to republish the same crate version.

The package include list excludes large benchmark artifacts and the local Cargo
CPU configuration. Benchmark records and source snapshots remain in Git and on
retained EC2 disks; they are not shipped to crates.io.
