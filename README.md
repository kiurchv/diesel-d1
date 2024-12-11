# Diesel D1 backend + connection

This is a custom backend/connection for [Diesel](https://diesel.rs/) for [D1](https://developers.cloudflare.com/d1/) targeting Cloudflare Workers, at the moment.

**IMPORTANT:** THIS IS NOT PRODUCTION READY YET, THINGS WILL PROBABLY BREAK (feel free to use it tho).

## Compatability

At the moment, this only supports Cloudflare Workers via the D1 binding (therefore, it only supports WASM). Generic support for the HTTP API is coming later.

## TO-DO List

- [ ] proper "transaction" support
- [ ] make it more SQLite compatible
- [ ] HTTP API (and allow other targets that do not use WASM)
- [ ] Durable Object sync SQLite support
