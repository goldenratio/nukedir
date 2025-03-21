# nukedir

> Delete directories(folders) recursively from a specified path

Consider below folder tree, you can use this CLI to remove all `Folder_1` directories from `User_folder`

```
User_folder
├── Folder_A
│   ├── Folder_1  (To remove)
│   ├── Folder_2
│   ├── Folder_3
│   └── Folder_4
├── Folder_B
│   ├── Folder_1  (To remove)
│   ├── Folder_5
│   ├── Folder_6
│   └── Folder_7
└── Folder_C
    ├── Folder_1  (To remove)
    ├── Folder_2
    ├── Folder_4
    └── Folder_5
```

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
