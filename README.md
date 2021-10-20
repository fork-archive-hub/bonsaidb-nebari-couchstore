# BonsaiDb

*Formerly known as [`PliantDb`](https://crates.io/crates/pliantdb). Not yet released on crates.io as `BonsaiDb`.*

![BonsaiDb is considered experimental and unsupported](https://img.shields.io/badge/status-experimental-blueviolet)
[![crate version](https://img.shields.io/crates/v/bonsaidb.svg)](https://crates.io/crates/bonsaidb)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/bonsaidb/Tests/main)](https://github.com/khonsulabs/bonsaidb/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/bonsaidb/coverage/badge.svg)](https://khonsulabs.github.io/bonsaidb/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/bonsaidb/main/bonsaidb/)

BonsaiDb aims to be a [Rust](https://rust-lang.org)-written, ACID-compliant, document-database inspired by [CouchDB](https://couchdb.apache.org/). While it is inspired by CouchDB, this project will not aim to be compatible with existing CouchDB servers, and it will be implementing its own replication, clustering, and sharding strategies.

## Project Goals

The high-level goals for this project are:

- ☑️ Be able to build a document-based database's schema using Rust types.
- ☑️ Run within your Rust binary, simplifying basic deployments.
- ☑️ Run as a local-only file-based database with no networking involved.
- ☑️ Run as a networked server using QUIC with TLS enabled by default
- Easily set up read-replicas between multiple servers.
- Easily run a highly-available quorum-based cluster across at least 3 servers
- ☑️ Expose a Publish/Subscribe eventing system
- Expose a Job queue and scheduling system -- a la [Sidekiq](https://sidekiq.org/) or [SQS](https://aws.amazon.com/sqs/)

## ⚠️ Status of this project

**You should not attempt to use this software in anything except for experiments.** This project is under active development (![GitHub commit activity](https://img.shields.io/github/commit-activity/m/khonsulabs/bonsaidb)), but at the point of writing this README, the project is too early to be used.

If you're interested in chatting about this project or potentially wanting to contribute, come chat with us on Discord: [![Discord](https://img.shields.io/discord/578968877866811403)](https://discord.khonsulabs.com/).

## Example

Check out [./bonsaidb/examples](./bonsaidb/examples) for examples. To get an idea of how it works, this is a simple schema:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Shape {
    pub sides: u32,
}

impl Collection for Shape {
    fn collection_name() -> Result<CollectionName, InvalidNameError> {
        CollectionName::new("khonsulabs", "shapes")
    }

    fn define_views(schema: &mut Schematic) -> Result<(), Error> {
        schema.define_view(ShapesByNumberOfSides)
    }
}

#[derive(Debug)]
struct ShapesByNumberOfSides;

impl View for ShapesByNumberOfSides {
    type Collection = Shape;

    type Key = u32;

    type Value = usize;

    fn version(&self) -> u64 {
        1
    }

    fn name(&self) -> Result<Name, InvalidNameError> {
        Name::new("by-number-of-sides")
    }

    fn map(&self, document: &Document<'_>) -> MapResult<Self::Key, Self::Value> {
        let shape = document.contents::<Shape>()?;
        Ok(Some(document.emit_key_and_value(shape.sides, 1)))
    }

    fn reduce(
        &self,
        mappings: &[MappedValue<Self::Key, Self::Value>],
        _rereduce: bool,
    ) -> Result<Self::Value, view::Error> {
        Ok(mappings.iter().map(|m| m.value).sum())
    }
}
```

After you have your collection(s) defined, you can open up a database and insert documents:

```rust
    let db =
        Database::<Shape>::open_local("view-examples.bonsaidb", &Configuration::default()).await?;

    // Insert a new document into the Shape collection.
    db.collection::<Shape>().push(&Shape::new(3)).await?;
```

And query data using the Map-Reduce-powered view:

```rust
let triangles = db
    .view::<ShapesByNumberOfSides>()
    .with_key(3)
    .query()
    .await?;
println!("Number of triangles: {}", triangles.len());
```

## Why write another database?

- Deploying highly-available databases is hard (and often expensive). It doesn't need to be.
- We are passionate Rustaceans and are striving for an ideal of supporting a 100% Rust-based deployment ecosystem for newly written software.
- Specifically for the founding author [@ecton](https://github.com/ecton), the idea for this design dates back to thoughts of fun side-projects while running my last business which was built atop CouchDB. Working on this project is fulfilling a long-time desire of his.

```rust
let triangles = db
    .view::<ShapesByNumberOfSides>()
    .with_key(3)
    .query()
    .await?;
println!("Number of triangles: {}", triangles.len());
```

## Feature Flags

No feature flags are enabled by default in the `bonsaidb` crate. This is
because in most Rust executables, you will only need a subset of the
functionality. If you'd prefer to enable everything, you can use the `full`
feature:

```toml
[dependencies]
bonsaidb = { version = "*", default-features = false, features = "full" }
```

- `full`: Enables `local-full`, `server-full`, and `client-full`.
- `cli`: Enables the `bonsaidb` executable.

### Local databases only

```toml
[dependencies]
bonsaidb = { version = "*", default-features = false, features = "local-full" }
```

- `local-full`: Enables `local`, `local-cli`, `local-keyvalue`, and
  `local-pubsub`
- `local`: Enables the [`local`](https://dev.bonsaidb.io/main/bonsaidb/local/) module, which re-exports the crate
  `bonsaidb-local`.
- `local-cli`: Enables the `StructOpt` structures for embedding database
  management commands into your own command-line interface.
- `local-pubsub`: Enables `PubSub` for `bonsaidb-local`.
- `local-keyvalue`: Enables the key-value store for `bonsaidb-local`.

### `BonsaiDb` server

```toml
[dependencies]
bonsaidb = { version = "*", default-features = false, features = "server-full" }
```

- `server-full`: Enables `server`, `server-websockets`, `server-keyvalue`,
  and `server-pubsub`
- `server`: Enables the [`server`](https://dev.bonsaidb.io/main/bonsaidb/server/) module, which re-exports the crate
  `bonsaidb-server`.
- `server-websockets`: Enables `WebSocket` support for `bonsaidb-server`.
- `server-pubsub`: Enables `PubSub` for `bonsaidb-server`.
- `server-keyvalue`: Enables the key-value store for `bonsaidb-server`.

### Client for accessing a `BonsaiDb` server

```toml
[dependencies]
bonsaidb = { version = "*", default-features = false, features = "client-full" }
```

- `client-full`: Enables `client`, `client-trusted-dns`,
  `client-websockets`, `client-keyvalue`, and `client-pubsub`
- `client`: Enables the [`client`](https://dev.bonsaidb.io/main/bonsaidb/client/) module, which re-exports the crate
  `bonsaidb-client`.
- `client-trusted-dns`: Enables using trust-dns for DNS resolution. If not
  enabled, all DNS resolution is done with the OS's default name resolver.
- `client-websockets`: Enables `WebSocket` support for `bonsaidb-client`.
- `client-pubsub`: Enables `PubSub` for `bonsaidb-client`.
- `client-keyvalue`: Enables the key-value store for `bonsaidb-client`.

## Developing BonsaiDb

### Pre-commit hook

Our CI processes require that some commands succeed without warnings or errors. To ensure that code you submit passes the basic checks, install the included [pre-commit](./git-pre-commit-hook.sh) hook:

```bash
./git-pre-commit-hook.sh install
```

Once done, tools including `cargo fmt`, `cargo doc`, and `cargo test` will all be checked before `git commit` will execute.

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are open-source. This repository is available under the [MIT License](./LICENSE-MIT) or the [Apache License 2.0](./LICENSE-APACHE).
