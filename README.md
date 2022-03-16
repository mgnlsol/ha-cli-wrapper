# ha-cli-wrapper
This is the HA command line tool published to npm. It is a pure wrapper around the ha-cli cli tool published to nexus. 
If the tool doesn't find ha-cli command it will configure .npmrc promting for nexus credentials.

The cli is developed in Rust. Make sure you install the rust toolchain via https://www.rust-lang.org/tools/install


# Build
```cargo build --release```


# NPM 
The CLI will be distributed via NPM and the npm package is located in the npm directory


# Releases
A github action workflow will automatically build the cli for linux, windows and mac and upload the binary when a new release is created.