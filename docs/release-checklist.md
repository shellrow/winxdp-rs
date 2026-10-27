# Release checklist

- Update versions and the changelog; confirm the supported upstream baseline.
- Run the build checks in [CONTRIBUTING.md](../CONTRIBUTING.md), including MSRV,
  default/all features, examples, documentation and the header inventory.
- Run [hardware tests](testing.md) on the configurations claimed by the release.
  Update the [validation summary](validation.md) without extending untested claims.
- Review package file lists, license notices, dependency changes and README links.
  Exclude local logs, certificates, binaries and environment-specific files.
- Verify the packaged crates and publish `winxdp-sys` before `winxdp`.
- Tag the reviewed revision and publish release notes that identify known limits.

Release and publication are manual maintainer operations.
