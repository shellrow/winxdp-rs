# winxdp-sys

Raw Windows x64 bindings for Microsoft XDP for Windows 1.4.0, API v3.
A static C bridge compiles the upstream inline implementation against vendored
headers. It links `ntdll`; it does not use the deprecated `xdpapi.dll` API.

Requires Rust 1.88+, MSVC and a Windows SDK. XDP is installed separately at runtime.
Use [winxdp](https://crates.io/crates/winxdp) for scoped buffers and socket ownership.

The `legacy-rules` feature enables deprecated `XdpCreateProgram` calls. Experimental
upstream APIs retain their experimental status. Callers of the raw API must uphold
pointer validity, buffer bounds, ring synchronization and resource lifetimes.

See the [API inventory](https://github.com/shellrow/winxdp/blob/main/docs/api-coverage.md)
and [compatibility guide](https://github.com/shellrow/winxdp/blob/main/docs/upstream-compatibility.md).

Licensed under MIT. Vendored Microsoft headers retain their notices in `vendor/LICENSE`.
