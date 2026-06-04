# CONTRIBUTING

## Building

You will need:
- A ctrader oauth application
- An auth0 application correctly configured

```shell
cp default.env dev.env

# Update with your secrets
nano dev.env            
source ./dev.env

# Build client
npm install
npm run build:client

# Run server
cargo run -p onlytrades
# Open browser to http://localhost:4200
```

## Architecture

The project is divided into the following folders, following an onion-inspired organizational strategy:

```shell
/client
  # This is a super basic web client. It was meant to be 
  # temporary while I worked on the server implementation
/crates
  /cmd
    # cmd is for entrypoints. These are binaries and cdylibs
    # Entrypoints cannot import code from other entrypoints
    /onlytrades
      # This is the main http-server binary for the project
  
  /platform
    # Platform contains packages that have business logic.
    # Platform packages can import from other platform packages or from kit
    /*

  /kit
    # Kit contains generic utilities. These can be thought of as a private crates.io
    # where each package should, conceptually, be something that could be published
    # without knowledge of the codebase.

    # Kit packages can import from other kit packages, but not from platform
    /*
```

The application is split into multiple crates to improve build times as Rust will build crates in parallel
and only rebuild crates if a crate has changed.

## Notes

This is a work in progress.

Much of this was experimentation so there are some poor abstractions and API 
designs for some of the packages - especially the packages associated with
the ctrader socket connection and the plugin system.
