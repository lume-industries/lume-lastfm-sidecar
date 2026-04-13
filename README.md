# lume-lastfm-sidecar

Fetches recent scrobbles and now-playing status from Last.fm.

Produces `LastfmPayload` payloads conforming to the VZGLYD sidecar channel ABI.

This sidecar is designed to be reusable. Any slide can depend on it via git and receive data payloads through the standard channel ABI.

## Poll Interval

Every 10 minutes.

## Payload Format

`LastfmPayload` serialized as JSON bytes.

## Environment Variables

| Variable | Description |
|---|---|
| `LASTFM_API_KEY` | Last.fm API key (required) |
| `LASTFM_USERNAME` | Last.fm username (required) |

## Usage

Build the sidecar:

```bash
cargo build --target wasm32-wasip1 --release
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
