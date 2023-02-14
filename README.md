# WP Engine CLI v2

An improved version of my earlier tool. This version is build with Rust whereas the previous version was built using JavaScript with node.
I have also opted to not use async which will speed up performance.

The purpose of this tool is to provide an interface with the wpengine API. I plan on creating a headless
version of the tool to be implemented as part of a pipeline or other similar use case.

[wpengine API reference](https://wpengineapi.com/reference)

## Installation

## Authentication

Running the CLI for the first time will prompt you to add a username and password for the wpengine API.

You can also use the `auth` command to update the credentials or manually add them.

## Sites

The `sites` command will allow you to list, add, update, or delete existing sites. Passing the 
`-H` flag will enable headless mode for using the CLI in a pipeline or as part of a script.

## Installs

## Accounts

## Users

## Roadmap

- [ ] Add bulk edit options.
- [ ] Integrate WordPress CLI. e.g: Updating WordPress Sites
- [ ] Complete all WP Engine API endpoints
- [x] Implement headless version so that the tool can be used in pipelines
