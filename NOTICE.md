# Notice

This crate is a standalone copy of `ext/tls` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_tls` 0.246.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions are pinned explicitly, because the crate is
  built outside the Deno monorepo.
- `rustls` is pinned to `=0.23.45` (upstream: `=0.23.40`). The newer release
  fixes a TLS 1.3 handshake issue. This is the only functional change.
