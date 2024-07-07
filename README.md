# Powermove Game
Powermove is a RPG meets Colony Management game in active development, sponsored by the [Neutron Grants Foundation](https://x.com/NeutronGrants/status/1809192252922814472).

This is a mono-repository which will eventually contain the following components:

- **Game Client**, written in Rust with the [Bevy game engine](https://bevyengine.org).
- **Game Server**, also written in Rust.
- **Library**, for types and procedures used across game client, server & tooling such as *procedural static content generation*.
- **Smart Contracts**, in CosmWasm Rust.

Note that `game/assets` are not stored in this repository as these are mostly binary, can be rather large, and thus inappropriate for GitHub. I am still trying to find an efficient storage solution, preferably involving IPFS and/or Arweave. In fact, I might move this entire repository to [Radicle](https://radicle.xyz/).

# Development Setup

1. Clone this repo.
2. As a temporary solution, [download the game assets](https://mega.nz/folder/k5Zn2YbI#gXyaq-i9EM8cIXjHNEzNHQ) and store them in a `game/assets` folder.
