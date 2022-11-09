# Guessing Game

## Cargo.lock

Tracks the exact version of dependenices so that Cargo will use only the versions of the dependencies you specified until you indicate otherwise.

## Update Dependencies

To manually update dependencies run:

```bash
cargo update
```

Note that this will still respect the value in your Cargo.toml file. So if you have a dependency with version `0.8.3` it will only update to the lates patch release for that version, e.g. `0.8.5` or whatever is latest for `0.8.*`.

## Documentation

Cargo can build documentation locally and display it in your browser, including docs for any dependencies.

```bash
cargo doc --open
```

