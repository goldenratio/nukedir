# nukedir

> Delete directory(folder) recursively from a specified path

### Usage

TODO


### Build

```sh
cargo build --release
```

## Install

### Pre-built Binaries
TODO

### NPM
TODO


## Releasing nukedir

```
cargo release <VERSION LEVEL> --execute --no-publish
```

Where `<VERSION LEVEL>` is one of `major`, `minor`, or `patch`

Next you need to manually make the release in github from the tag. This will kick off the build process
to build all the releases assets and store them on the release in github. 
